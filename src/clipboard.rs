use clipboard_rs::{Clipboard, ClipboardContext};

pub fn clipboard(text: String) {
    let clx = ClipboardContext::new().unwrap();
    clx.set_text(text).unwrap()
}
