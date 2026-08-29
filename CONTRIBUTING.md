# Contributing to MusicMotion

Thank you for your interest in contributing to **MusicMotion**! 🎉

## 🛠️ Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/ItsSp00ky/music_motion.git
   cd music_motion
   ```

2. Install dependencies:
   ```bash
   bun install # or npm install
   ```

3. Run in development mode:
   ```bash
   bun run tauri dev
   ```

## 🧪 Testing and Quality

Before submitting a Pull Request, ensure:
- Rust backend compiles without errors or warnings:
  ```bash
  cargo check --manifest-path src-tauri/Cargo.toml
  ```
- Frontend builds and passes TypeScript checks:
  ```bash
  bun run build
  ```

## 📬 Pull Request Guidelines

1. Create a feature branch: `git checkout -b feature/my-feature`
2. Commit your changes: `git commit -m "feat: add my new feature"`
3. Push to your branch: `git push origin feature/my-feature`
4. Open a Pull Request on GitHub.
