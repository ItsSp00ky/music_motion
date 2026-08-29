# 🎵 MusicMotion

<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="MusicMotion Logo" width="96" height="96" />
</p>

<p align="center">
  <strong>Ultra-lightweight, high-performance transparent audio & music overlay for Windows.</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#themes">Themes</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#custom-themes">Custom Themes</a> •
  <a href="#building-from-source">Building from Source</a> •
  <a href="LICENSE">License</a>
</p>

---

## ✨ Features

- **⚡ Blazing Fast & Lightweight**: Native Rust backend powered by Tauri v2 with **<20MB RAM** usage and **~0% idle CPU**.
- **🔍 Hybrid Audio Detection Engine**:
  - **Windows Media (GSMTC / WinRT)**: Real-time track title, artist, album art, and playback status from Spotify, Apple Music, YouTube (Chrome/Edge/Firefox), Tidal, VLC, and foobar2000.
  - **WASAPI Session Peak Metering**: Real-time volume meter detecting any active sound-producing application (games, Discord voice calls, browser tabs, media players).
- **🎨 Modular 60 FPS Visualizer**: Fluid equalizer spectrum bars and smooth spline waveforms that react dynamically to sound volume.
- **✨ Frosted Glass Acrylic UI**: Translucent modern card floating above your Windows taskbar with smooth slide-up and fade animations.
- **🛡️ Click-Through Mode**: Toggle click-through with a single click so your mouse passes right through the overlay to underlying windows without disruption.
- **⚙️ Complete System Tray Controls**: Customize screen anchor positions (Bottom-Right, Bottom-Left, Top-Right, Top-Left), switch themes, toggle click-through, and open custom themes folder.
- **🌙 Smart Idle Auto-Hide**: Automatically fades away into transparency when music or sound stops, waking up instantly when audio resumes.
- **📦 Extensible Community Theme Engine**: Easily build and drop in custom themes using HTML/CSS/JS or JSON styling manifests.

---

## 🎨 Built-in Themes

| Theme | Preview Description |
|---|---|
| **Frosted Fluent Card** *(Default)* | Modern acrylic glassmorphism card with album art glow, scrolling marquee title, app badge, and spectrum equalizer. |
| **Minimal HUD** | Ultra-compact translucent pill badge with mini equalizer bars and app indicator. |
| **Dynamic Island** | Sleek morphing pill that dynamically expands when music plays and smoothly collapses when idle. |
| **Cyber Neon** | High-contrast glowing cyberpunk neon card with reactive sound spectrum. |

---

## 🚀 Installation

### Option 1: Download Pre-built Release (Recommended)
1. Go to the [Releases](https://github.com/your-username/music-motion/releases) page.
2. Download the latest `MusicMotion_x64_en-US.msi` installer or portable `.exe`.
3. Run the installer and launch **MusicMotion**.

### Option 2: Run with Bun / Cargo
```powershell
# Clone the repository
git clone https://github.com/your-username/music-motion.git
cd music_motion

# Install frontend dependencies
bun install   # or npm install

# Run in development mode (starts overlay above taskbar)
bun run tauri dev
```

---

## 🎮 Usage & Controls

### System Tray
Look for the 🎵 icon in your Windows System Tray (near the clock):
- **Click-Through Mode**: Lock or unlock mouse clicks through the overlay window.
- **Screen Position**: Move the overlay between Bottom-Right, Bottom-Left, Top-Right, or Top-Left.
- **Themes**: Switch between built-in themes on the fly.
- **Open Themes Folder**: Jump directly to your custom themes directory.

### In-Window Context Menu
When Click-Through mode is disabled:
- **Right-Click** anywhere on the overlay to bring up the quick menu to toggle Click-Through or cycle through themes.

---

## 🖌️ Creating Custom Themes

MusicMotion supports custom themes stored in your user configuration directory:
`%APPDATA%\MusicMotion\themes`

Check out our [THEMES.md](THEMES.md) guide for creating and publishing community themes.

---

## 🛠️ Building from Source

### Prerequisites
- [Rust Toolchain (1.80+)](https://rustup.rs/)
- [Node.js (v18+)](https://nodejs.org/) and [Bun](https://bun.sh/) (or `npm`)
- [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Build Steps
```powershell
# Build production bundle
bun run tauri build
```
The compiled standalone executable and `.msi` installer will be located in:
`src-tauri/target/release/bundle/`

---

## 🏗️ Architecture

```
music_motion/
├── src/                        # Frontend UI & Visualizer Engine (TypeScript / Canvas / CSS)
│   ├── themes/                 # Built-in themes (frosted-card, minimal-hud, dynamic-island, cyber-neon)
│   ├── visualizer/             # 60fps Canvas audio waveform & equalizer engine
│   ├── index.html              # Transparent overlay container
│   ├── main.ts                 # Tauri IPC event bridge & auto-hide loop
│   └── style.css               # Frosted acrylic & theme styling
├── src-tauri/                  # Native Rust Core (Tauri v2)
│   ├── src/
│   │   ├── audio/              # WASAPI audio peak enumerator + GSMTC WinRT media tracker
│   │   ├── window/             # Win32 taskbar positioning & WS_EX_TRANSPARENT click-through
│   │   ├── config.rs           # Persistent user settings (%APPDATA%/MusicMotion)
│   │   ├── tray.rs             # System tray menu and handlers
│   │   └── lib.rs              # Tauri command handlers & background audio loop
│   ├── Cargo.toml
│   └── tauri.conf.json         # Transparent, frameless, always-on-top window setup
└── .github/workflows/          # Automated GitHub Actions CI/CD release workflow
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
