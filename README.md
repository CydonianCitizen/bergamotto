# 🍋 Bergamotto

**The open-source, cross-platform radial action launcher for your files.**  
*Inspired by Tangerine. Built with Rust & Tauri v2. 100% customizable.*

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)]()
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust%20%26%20Tauri%20v2-orange)]()

**Bergamotto** opens a radial wheel of customizable actions for your selected files.

Unlike traditional radial converters that lock you into a hardcoded list of file format conversions, **Bergamotto has zero hardcoded actions**. Every wedge on the wheel is powered by a user-editable JSON manifest: run CLI tools (`ffmpeg`, `imagemagick`, `pandoc`), trigger custom Python/Bash/PowerShell scripts, upload files to HTTP webhooks, or launch custom workflows.

---

## ✨ Why Bergamotto?

* **🧩 100% Customizable Actions:** No fixed conversion matrices. Create, edit, duplicate, or share actions via simple JSON files or through the built-in **Action Studio** UI.
* **⚡ Hot-Reloading Registry:** Edit an action manifest in your favorite text editor and see the radial wheel update instantly—no app restart required.
* **🌍 Truly Cross-Platform:**
  * **macOS:** Universal binary supporting both **Apple Silicon (M-series)** and **Intel (`x86_64`)**.
  * **Windows:** Native Win32/Explorer drag interception (`x64`).
  * **Linux:** Native X11 drag detection + CLI/Shortcut trigger (`bergamot --radial-here`) for Wayland compositors (GNOME, KDE, Hyprland, Sway).
* **🦀 Lightweight & Safe:** Powered by a single unified **Rust** core (`bergamot-core`) and **Tauri v2** (using native OS webviews instead of bundling Chromium).
* **🛡️ Process-Tree & File Safety:**
  * External commands run without shell interpolation (`shell=False`) to prevent injection bugs on filenames with spaces or special characters.
  * Cancelling a job terminates the entire child process tree (via Unix Process Groups and Windows Job Objects).
  * File outputs use atomic staging so interrupted tasks never corrupt or overwrite your original files.

---

## 🚀 How It Works

1. **Select files** with **Test on Wheel** in Action Studio, drop them into Studio, or pass them to `bergamot --radial-here`.
2. **Choose a slice** to run its action. With an empty wheel open from the tray, you can drag files directly onto a slice.
3. Follow progress in the floating HUD.

---

## 🛠️ Customizing Actions

Every action in Bergamotto lives as a standalone `.json` file inside your OS configuration folder:
* **macOS:** `~/Library/Application Support/Bergamot/actions/`
* **Windows:** `%APPDATA%\Bergamot\actions\`
* **Linux:** `~/.config/bergamot/actions/`

### Example 1: CLI Action (Compress Video via FFmpeg)
```json
{
  "schemaVersion": 1,
  "id": "user.ffmpeg-compress",
  "label": "Compress MP4",
  "description": "Compress video using H.264 CRF 28",
  "icon": "film",
  "enabled": true,
  "platforms": ["macos", "windows", "linux"],
  "matching": {
    "extensions": ["mp4", "mov", "mkv"],
    "cardinality": "each"
  },
  "executor": {
    "type": "process",
    "program": "ffmpeg",
    "args": [
      "-i", "${input}",
      "-vcodec", "libx264",
      "-crf", "28",
      "${output}"
    ],
    "timeoutSeconds": 600
  },
  "output": {
    "mode": "file",
    "extension": "mp4",
    "suffix": "-compressed",
    "overwriteOriginal": false
  }
}
```

### Example 2: Webhook Action (Upload to Custom API / Discord / S3)
```json
{
  "schemaVersion": 1,
  "id": "user.upload-webhook",
  "label": "Send to Webhook",
  "description": "Upload selected files to a remote endpoint",
  "icon": "upload-cloud",
  "enabled": true,
  "platforms": ["macos", "windows", "linux"],
  "matching": {
    "extensions": ["*"],
    "cardinality": "batch"
  },
  "executor": {
    "type": "webhook",
    "method": "POST",
    "url": "https://api.example.com/upload",
    "fileFieldName": "file",
    "credentialKey": "my-api-token",
    "timeoutSeconds": 120
  },
  "output": {
    "mode": "clipboard",
    "overwriteOriginal": false
  }
}
```

(Secrets like `credentialKey` are stored safely in macOS Keychain, Windows Credential Manager, or Linux Secret Service—never in plaintext JSON).

---

## 🏗️ Architecture

```text
Bergamotto
├── shared/                 # JSON Schemas, starter action templates, and test fixtures
├── crates/bergamot-core    # Pure Rust engine: CRUD Registry, Matcher, Process/Script/Webhook Executors
├── crates/os-adapter       # Native OS drag hooks (AppKit Pasteboard, Win32 COM, X11/CLI)
├── src-tauri               # Tauri v2 window manager & IPC bridge
└── src                     # React + TypeScript + Tailwind UI (Radial Wheel, HUD, Action Studio)
```

---

## 📦 Building from Source

### Prerequisites
* Rust (stable toolchain)
* Node.js (20.19+ or 22.13+)
* Platform-specific Tauri v2 dependencies (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

### Development
```bash
git clone https://github.com/yourusername/bergamotto.git
cd bergamotto
npm install
npm run tauri dev
```

Open **Test on Wheel** in Action Studio to choose sample files, or drop files onto the Studio window. The tray's **Test Radial Wheel** item opens an empty wheel for a file drop.

With a development build running, this command forwards files to the wheel. It also works on first launch.

```bash
./target/debug/bergamot --radial-here "/path/to/sample file.txt"
```

On macOS, Windows, and X11, hold **Shift** while dragging files to open the wheel under the cursor, then drop on a slice. Release **Shift** to dismiss the wheel mid-drag. Use **Global Shift+Drag Hook** in the tray or Action Studio header to pause or resume detection.

On Wayland, use `bergamot --radial-here <files...>`. The compositor controls where the wheel appears.

### Run Core Tests
```bash
cargo test --workspace
```

---

## 📄 License

Distributed under the GPL-3.0 License. See [LICENSE](LICENSE) for more information.
