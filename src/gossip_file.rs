use anyhow::Result;
use futures_lite::StreamExt;
use iroh::{Endpoint, protocol::Router};
use iroh::{NodeAddr, NodeId};
use iroh_gossip::net::{Gossip, GossipSender};
use iroh_gossip::{
    net::{Event, GossipEvent, GossipReceiver},
    proto::TopicId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

use crate::clipboard::clipboard;

// Message enum (same as before)
#[derive(Debug, Serialize, Deserialize)]
enum Message {
    AboutMe {
        from: NodeId,
        name: String,
    },
    Message {
        from: NodeId,
        text: String,
        timestamp: u64,
    },
}

impl Message {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serialization failed")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Ticket {
    topic: TopicId,
    nodes: Vec<NodeAddr>,
}

impl Ticket {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text = data_encoding::BASE32_NOPAD.encode(&self.to_bytes());
        text.make_ascii_lowercase();
        write!(f, "{}", text)
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = data_encoding::BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        Ticket::from_bytes(&bytes)
    }
}

pub async fn open_gossip_room(name: Option<String>) -> Result<()> {
    let topic = TopicId::from_bytes(rand::random());
    println!("> Opening gossip room for topic: {topic}");

    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    println!("> Our node id: {}", endpoint.node_id());

    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;
    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    let me = endpoint.node_addr().await?;
    let ticket = Ticket {
        topic,
        nodes: vec![me],
    };

    println!("> Share this ticket to join: {ticket}");
    clipboard(ticket.to_string());

    let (sender, receiver) = gossip.subscribe_and_join(topic, vec![]).await?.split();

    if let Some(name) = name.clone() {
        let message = Message::AboutMe {
            from: endpoint.node_id(),
            name,
        };
        sender.broadcast(message.to_vec().into()).await?;
    };

    tokio::spawn(subscribe_loop(
        receiver,
        endpoint.clone(),
        name,
        sender.clone(),
    ));

    let (line_tx, mut line_rx) = mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));

    println!("> Type messages and hit enter to send:");

    while let Some(text) = line_rx.recv().await {
        let message = Message::Message {
            from: endpoint.node_id(),
            text: text.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        sender.broadcast(message.to_vec().into()).await?;
        println!("> Sent: {text}");
    }

    router.shutdown().await?;
    Ok(())
}

pub async fn join_gossip_room(ticket_str: String, name: Option<String>) -> Result<()> {
    let Ticket { topic, nodes } = Ticket::from_str(&ticket_str)?;
    println!("> Joining room with topic: {topic}");

    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    println!("> Our node id: {}", endpoint.node_id());

    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;
    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    for node in &nodes {
        endpoint.add_node_addr(node.clone())?;
    }

    let node_ids = nodes.iter().map(|p| p.node_id).collect();
    let (sender, receiver) = gossip.subscribe_and_join(topic, node_ids).await?.split();

    println!("> Connected to gossip topic.");

    if let Some(name) = name.clone() {
        let message = Message::AboutMe {
            from: endpoint.node_id(),
            name,
        };
        sender.broadcast(message.to_vec().into()).await?;
    }

    tokio::spawn(subscribe_loop(
        receiver,
        endpoint.clone(),
        name,
        sender.clone(),
    ));

    let (line_tx, mut line_rx) = mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));

    println!("> Type messages and hit enter to send:");

    while let Some(text) = line_rx.recv().await {
        let message = Message::Message {
            from: endpoint.node_id(),
            text: text.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        sender.broadcast(message.to_vec().into()).await?;
        println!("> Sent: {text}");
    }

    router.shutdown().await?;
    Ok(())
}

async fn subscribe_loop(
    mut receiver: GossipReceiver,
    endpoint: Endpoint,
    my_name: Option<String>,
    sender: GossipSender,
) -> Result<()> {
    let mut names = HashMap::new();

    while let Some(event) = receiver.try_next().await? {
        if let Event::Gossip(GossipEvent::NeighborUp(node_id)) = &event {
            println!("got triggered by: {}", node_id);
            let message = Message::AboutMe {
                from: endpoint.node_id(),
                name: match my_name.clone() {
                    Some(name) => name,
                    None => "Anonymous".to_string(),
                },
            };
            sender.broadcast(message.to_vec().into()).await?;
        };

        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            match Message::from_bytes(&msg.content)? {
                Message::AboutMe { from, name } => {
                    names.insert(from, name.clone());
                    println!("> {} is now known as {}", from.fmt_short(), name);
                }
                Message::Message {
                    from,
                    text,
                    timestamp: _,
                } => {
                    let name = names
                        .get(&from)
                        .map_or_else(|| from.fmt_short(), String::to_string);
                    println!("{}: {}", name, text);
                }
            }
        };
    }

    Ok(())
}

fn input_loop(line_tx: mpsc::Sender<String>) -> Result<()> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();

    for line in stdin_lock.lines() {
        let text = line?;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("{}", time);
        line_tx.blocking_send(text)?;
    }

    Ok(())
}
