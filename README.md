# 🚪 zerogate

**`zerogate`** is a minimal, cross-platform command-line tool focused on performance and simplicity.  
One binary. Zero setup headaches.

---

## 🔧 Installation

Install `zerogate` globally in **one command**—no manual steps required!

---

### 🪟 Windows (PowerShell)

> ⚠️ **Make sure to run PowerShell as Administrator**

Run the following command to automatically install `zerogate`:

```powershell
irm https://raw.githubusercontent.com/kayesFerdous/p2p/main/install.ps1 | iex
```

What this does:

- Downloads the latest version of `zerogate.exe`  
  ➤ [v1.1.0 Release](https://github.com/kayesFerdous/p2p/releases/download/v1.1.0/zerogate.exe)
- Creates the folder `C:\ZerogateTool` if it doesn't exist
- Moves the binary there
- Adds that folder to your system `PATH` (if it's not already)

✅ After installation, **restart your terminal** and verify it:

```powershell
zerogate --version
```

---

### 🐧 Linux (Bash)

> 🧑‍💻 Works on **Ubuntu**, **Debian**, **Arch**, **Fedora**, and most major distros.

Run the following command in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/kayesFerdous/p2p/main/install.sh | bash
```

What this does:

- Downloads the latest `zerogate` binary  
  ➤ [v1.1.0 Release](https://github.com/kayesFerdous/p2p/releases/download/v1.1.0/zerogate)
- Moves it to `/usr/local/bin/zerogate`
- Sets the correct executable permissions

✅ Once the script finishes, check it with:

```bash
zerogate --version
```

---

## 📦 Features

- ✅ Zero configuration required
- 🚀 Lightning-fast performance
- 🔁 Seamless cross-platform support
- 🧩 Single static binary

---

## 📄 License

Licensed under the [MIT License](https://opensource.org/licenses/MIT).

---

Made with ❤️ by [@kayesFerdous](https://github.com/kayesFerdous)
