# 🍋 Bergamotto

**The open-source, cross-platform radial action launcher for your files.**  
*Inspired by Tangerine. Built with Rust & Tauri v2. 100% customizable.*

[![Release](https://img.shields.io/badge/version-0.0.1--alpha-blue)](https://github.com/cydoniancitizen/bergamotto/releases)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)]()
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust%20%26%20Tauri%20v2-orange)]()

**Bergamotto** (`v0.0.1-alpha`) opens a radial wheel of customizable actions for your selected files.

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

`credentialKey` names a token in macOS Keychain, Windows Credential Manager, or Linux Secret Service. Enter or remove the token in the webhook editor. Action JSON and exports carry the key name. Configure the token separately on each machine. Linux requires an available, unlocked D-Bus Secret Service provider for credential operations.

### Settings & Paths

Open **Settings & Paths** in Action Studio to choose single or dual modifier keys (`Shift`, `Ctrl`, `Alt`, `Alt + Shift`, `Ctrl + Shift`, `Ctrl + Alt`) for the global hook. The tray and Studio share the toggle, and both the modifier and toggle persist in `settings.json`. On macOS, Alt is Option.

For a missing tool, add an absolute executable directory or an override from the manifest's tool name to the absolute executable path. Bergamotto searches custom directories first, then platform defaults (`/opt/homebrew/bin` and `/usr/local/bin` on macOS, `~/.local/bin` on Linux), then `PATH`, and finally managed tools. Overrides take precedence. Studio checks availability with the same resolver used to launch the action.

### Managed Tools (Fallback Tooling)

Bergamotto adheres to a zero-bundled-binary philosophy: installers remain tiny, lightweight, and completely free of third-party executables. However, when you run an action that requires a missing tool (such as FFmpeg, Python 3, or ImageMagick), Bergamotto offers a **Managed Tools** system:

* **On-Demand Consent:** When you drop files onto an action whose CLI tool is not installed on your system, Bergamotto prompts you with a consent dialog displaying the tool version, estimated download size, and its exact open-source license terms (with links to the license text and upstream source code). You can choose to download and install it automatically, use your existing binary, or cancel.
* **Strict Resolution Order:** Managed tools are always the **last fallback**. Bergamotto checks user tool overrides, custom paths, platform default folders, and system `PATH` first. If you already have `ffmpeg` or `python3` in your `PATH`, Bergamotto uses your system version and will never download a duplicate.
* **Security & Integrity:**
  * Downloads are performed strictly over HTTPS and hop-by-hop validated against an exact domain allowlist (`github.com`, `release-assets.githubusercontent.com`, `johnvansickle.com`, `ffmpeg.martin-riedl.de`, `evermeet.cx`, `download.imagemagick.org`).
  * Cryptographic **SHA-256 digests** are checked before decompressing or executing any file.
  * Archives are extracted with strict path traversal (Zip-slip) protection, link escape guards, file count limits, and uncompressed size ceilings.
  * After extraction, every tool is actively verified: FFmpeg must have `--enable-gpl`, must not contain `--enable-nonfree`, and must include `libx264` and `libmp3lame`; Python 3 runs a test suite importing standard library modules (`hashlib`, `zipfile`, `pathlib`); ImageMagick verifies WebP delegate support.
* **Installation Directory:**
  * **macOS:** `~/Library/Application Support/Bergamot/tools/<tool>/<version>/`
  * **Windows:** `%LOCALAPPDATA%\Bergamot\tools\<tool>\<version>\`
  * **Linux:** `~/.local/share/bergamot/tools/<tool>/<version>/`
* **Uninstallation:** You can remove any downloaded tool at any time directly from Action Studio in **Settings & Paths** > **Managed Tools** by clicking **Remove**, or by deleting the tool's directory.

#### Managed Tools Reference

| Tool | Source & Platform Support | License | Verification Checks |
| --- | --- | --- | --- |
| **FFmpeg** | Windows x64 (GyanD 7.1), Linux x64/arm64 (John Van Sickle 7.0.2 static), macOS arm64 (Martin Riedl 7.1), macOS x64 (Evermeet 9.0.2) | GPL-3.0 (copyleft) | `ffmpeg -version` (verifies `--enable-gpl`, rejects `--enable-nonfree`), `-encoders` (`libx264`, `libmp3lame`) |
| **Python 3** | Windows x64, Linux x64/arm64, macOS arm64/x64 (`astral-sh/python-build-standalone` CPython 3.12.16) | PSF-2.0 (bundled third-party licenses) | `python3 --version`, `-c "import hashlib, zipfile, pathlib; print('OK')"` |
| **ImageMagick** | Windows x64 (portable 7.1.2 7z), Linux x64 (7.1.2 AppImage executed via `AppRun`). macOS and Linux arm64 use system package managers (`brew`, `apt`, `pacman`, `dnf`). | ImageMagick (Apache-2.0 compatible) | `magick -version`, `-list format` (verifies `WEBP` read/write) |

### Action Packs

In **Action Packs**, export one saved action or all enabled actions to a `.bergamot-pack.json` file or the clipboard. Import accepts either a pack array or a single action JSON. Studio previews matching IDs and lets you overwrite them or keep both. Inspect imported commands and webhook URLs before running them.

New config directories get six editable starters. Existing action files remain as saved.

| Starter | Requirement and result |
| --- | --- |
| Compress video to MP4 | FFmpeg with libx264. Writes an H.264 MP4 beside each input. |
| Extract audio to MP3 | FFmpeg with libmp3lame. Writes the first audio stream as MP3. |
| Convert image to WebP | ImageMagick 7 with WebP support. Writes one WebP per input. |
| Copy SHA-256 checksums | Python 3. Copies one digest and filename per input. |
| Archive files to ZIP | Python 3. Writes one archive beside the first input. |
| Upload to webhook | An HTTP endpoint accepting multipart POST. Configure the URL before enabling. |

On Windows, `python3` automatically maps to your system `python.exe` or to the managed Python fallback, so manual overrides are optional.

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

#### Arch Linux
Install system dependencies via `pacman`:
```bash
sudo pacman -S --needed base-devel webkit2gtk-4.1 libayatana-appindicator librsvg libx11 dbus openssl
```

Build from source:
```bash
git clone https://github.com/yourusername/bergamotto.git
cd bergamotto
npm install
npm run tauri build   # or `npm run tauri dev` for live development
```

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

### OS & Wayland Setup Guide

* **macOS:** Hold `Shift` (or configured modifier) while dragging files in Finder. Bergamotto requires Accessibility / Input Monitoring permissions when prompted to read global drag coordinates.
* **Windows:** Hold `Shift` while dragging files in File Explorer (`CabinetWClass` / `ExploreWClass`). Bergamotto automatically queries active shell selection using native Windows COM (`IShellWindows`).
* **Linux (X11):** Hold `Shift` while dragging files in any X11 file manager. Global pointer and modifier polling detects the drag threshold and centers the wheel under your cursor.
* **Linux (Wayland):** Because Wayland protocols deliberately isolate pointer and keyboard events between windows for security, background drag interception is restricted. Bergamotto provides first-class CLI IPC integration:
  * Run `bergamot --radial-here <files...>` to open the wheel with your selection.
  * Run `bergamot --radial-clipboard` to open the wheel with files copied to the clipboard.
  * **File Manager Integration:**
    * **GNOME / Nautilus:** Add a Nautilus script in `~/.local/share/nautilus/scripts/Open with Bergamotto`:
      ```bash
      #!/bin/bash
      bergamot --radial-here "$@"
      ```
    * **KDE / Dolphin:** Create a Dolphin service menu in `~/.local/share/kio/servicemenus/bergamot.desktop`:
      ```ini
      [Desktop Entry]
      Type=Service
      MimeType=all/allfiles;
      Actions=openBergamot;
      X-KDE-Priority=TopLevel

      [Desktop Action openBergamot]
      Name=Open in Bergamotto Wheel
      Icon=bergamotto
      Exec=bergamot --radial-here %F
      ```
    * **Thunar / Nemo / PCManFM:** Add a custom action executing `bergamot --radial-here %F`.
  * **Compositor Hotkey (e.g. `Alt + Shift + Space`):**
    Bind a global shortcut in your compositor (Hyprland, Sway, KDE, GNOME) to trigger Bergamotto with clipboard files:
    * **Hyprland:** `bind = ALT SHIFT, space, exec, bergamot --radial-clipboard`
    * **Sway:** `bindsym Mod1+Shift+space exec bergamot --radial-clipboard`
    * **KDE / GNOME:** Add a custom shortcut bound to `Alt+Shift+Space` running `bergamot --radial-clipboard`.

### CLI Usage
```bash
# Print version
bergamot --version    # Outputs: Bergamotto 0.0.1-alpha
bergamot -V

# Open radial wheel with selected files
bergamot --radial-here "/path/to/file1.png" "/path/to/file2.png"

# Open radial wheel with clipboard files (ideal for Wayland global hotkeys)
bergamot --radial-clipboard
```

---

## 🗺️ Project Roadmap

- [x] **Milestone 1:** Core Rust Engine (`bergamot-core`), JSON Manifest Schemas, Process/Script/Webhook Execution & Atomic Staging.
- [x] **Milestone 2:** Tauri v2 Shell, Dynamic SVG Radial Menu, Floating HUD Progress Island, Action Studio CRUD.
- [x] **Milestone 3:** Global OS Drag Interception (macOS Pasteboard, Windows Explorer COM, Linux X11/Wayland CLI).
- [x] **Milestone 4:** Action Packs (.bergamot-pack.json import/export), Keychain/Credential Store Integration, Settings & Custom Tool Paths.
- [x] **Milestone 5:** Versioning (`0.0.1-alpha`), Cross-Platform GitHub Actions CI/CD, Application Icons, and Release Packaging (`.dmg`, `.deb`, `.AppImage`, `.exe`, `.msi`).
- [x] **Milestone 6:** Visual Wheel Customizer & Guided Action Builder GUI:
  - Drag-and-drop wedge angle and ordering visualizer.
  - Guided step-by-step No-Code action builder for CLI tools, scripts, and webhooks.
  - Live interactive wheel preview inside Action Studio with simulated drop targets.

---

## 📄 License

Distributed under the GPL-3.0 License. See [LICENSE](LICENSE) for more information.
