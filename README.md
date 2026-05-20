# 🔥 Reliquary

> **The default GUI file manager for [Lilith Linux](https://github.com/BlancoBAM)**
> Built on [filedime](https://github.com/visnkmr/filedime) · Powered by Tauri + Next.js

![Lilith Linux](https://img.shields.io/badge/Lilith_Linux-Default_File_Manager-c0392b?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-1.x-blueviolet?style=for-the-badge)
![License](https://img.shields.io/github/license/BlancoBAM/Reliquary?style=for-the-badge)

---

## ✨ Features

| Feature | Description |
|---|---|
| **Drag & Drop Move** | Drag any file/folder onto a directory to move it instantly |
| **Undo (Ctrl+Z)** | Undo the last move, copy, rename, or create operation |
| **Rename** | Right-click → Rename, or use `F2` |
| **Delete / Trash** | Right-click → Move to Trash (XDG-compliant) or Delete Permanently |
| **Copy / Cut / Paste** | Full clipboard-style file operations with conflict resolution |
| **Tabs & Multi-window** | Multiple tabs and independent windows |
| **Bookmarks** | Sidebar bookmarks for quick access |
| **Drive Listing** | Auto-detect and browse mounted drives |
| **File Preview** | Inline preview of text, images, video, PDF, Office files |
| **Search** | Full-path fuzzy search with search-list indexing |
| **AI File Queries** | Ask questions about file contents via [Lilim](https://github.com/BlancoBAM/Lilim) |
| **Miller Columns** | Alternative column-based navigation view |
| **Dual Viewer** | Side-by-side file comparison |
| **Dark Theme** | Lilith Linux infernal dark theme by default |

---

## 🎨 Theming

Reliquary ships with the **Lilith Linux Infernal Dark** theme:

- **Background**: `#0a0a0a` (near-black)
- **Surface**: `#111111` / `#1a1a1a`
- **Primary accent**: `#c0392b` (crimson flame)
- **Secondary accent**: `#ff6b35` (ember orange)
- **Typography**: Inter + Rajdhani (via Google Fonts)

---

## 🤖 AI Integration (Lilim)

Reliquary connects to [Lilim](https://github.com/BlancoBAM/Lilim) — or any Ollama-compatible server — for:

- **File embedding** — index file contents as vectors
- **Semantic search** — query your files in natural language
- **File chat** — ask questions about a specific document

Configure the server URL in **Settings → Lilim / LLM server URL** (default: `http://127.0.0.1:11434`).

---

## 🚀 Building

### Prerequisites

```bash
# Ubuntu / Lilith Linux
sudo apt install libwebkit2gtk-4.0-dev build-essential libssl-dev libgtk-3-dev
# Node 20+, Rust stable
```

### Dev mode

```bash
npm install
npm run dev   # starts Next.js + Tauri dev server
```

### Release build

```bash
npm run build
npx tauri build
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+Z` | Undo last file operation |
| `Delete` | Move selected to Trash |
| `Shift+Delete` | Permanently delete selected |
| `Ctrl+C` | Copy current path/selection |
| `Ctrl+X` | Cut current path/selection |
| `Ctrl+V` | Paste (drop to current folder) |
| `Ctrl+T` | New tab |
| `Ctrl+N` | New window |
| `Ctrl+W` | Close tab |
| `Ctrl+H` | Toggle hidden files |
| `Alt+←` | Navigate back |
| `Alt+→` | Navigate forward |
| `Alt+↑` | Navigate to parent |
| `F5` | Refresh |

---

## 📦 Installing on Lilith Linux

Reliquary replaces `cosmic-files` as the default file manager. AppImage releases are built automatically via GitHub Actions on every version tag.

```bash
# Download latest AppImage from Releases
chmod +x Reliquary_*.AppImage
./Reliquary_*.AppImage
```

---

## 🙏 Credits

- Original **filedime** by [visnkmr](https://github.com/visnkmr/filedime) — MIT License
- Reliquary fork & Lilith Linux integration by **BlancoBAM**