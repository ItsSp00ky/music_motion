# 🎨 MusicMotion Theme Creation Guide

MusicMotion features a modular theme architecture that allows anyone to design and share custom overlay themes.

---

## 📂 Theme Directory Structure

Custom themes reside in your MusicMotion user directory:
`%APPDATA%\MusicMotion\themes\<your-theme-name>\`

Each custom theme folder contains:
```
my-custom-theme/
├── manifest.json       # Metadata and layout parameters
└── theme.css           # Custom CSS styles, animations, and visualizer colors
```

---

## 📋 `manifest.json` Format

```json
{
  "id": "my-custom-theme",
  "name": "My Custom Theme",
  "version": "1.0.0",
  "author": "YourName",
  "description": "A stylish custom visualizer overlay",
  "width": 380,
  "height": 130,
  "visualizer": {
    "type": "bars",
    "primaryColor": "#38bdf8",
    "secondaryColor": "#818cf8"
  }
}
```

### Visualizer Options:
- `"type"`: `"bars"` | `"wave"` | `"dots"`
- `"primaryColor"`: Any valid CSS color (`#hex`, `rgba(...)`)
- `"secondaryColor"`: Secondary gradient color

---

## 🎨 Example `theme.css`

```css
.theme-custom {
  position: relative;
  width: 100%;
  height: 100%;
  background: rgba(15, 23, 42, 0.8);
  backdrop-filter: blur(24px);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 16px;
  padding: 12px;
  display: flex;
  align-items: center;
  gap: 12px;
}

.theme-custom .track-title {
  font-size: 14px;
  font-weight: 700;
  color: #38bdf8;
}

.theme-custom .artist-name {
  font-size: 12px;
  color: #94a3b8;
}
```

---

## 📤 Sharing Themes with the Community

1. Fork the [MusicMotion Repository](https://github.com/your-username/music-motion).
2. Place your theme folder under `themes/<your-theme-name>/`.
3. Submit a Pull Request!
