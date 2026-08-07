# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Language**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` is a lightweight, cross-platform CLI tool written in **Rust**. It eliminates the hassle of manually entering `/resume` every time you launch AI CLI coding assistants (such as **Google AGY CLI** or **GitHub Copilot CLI**) in your project workspace.

---

## 🌟 Key Features

- **⚡ Automatic Workspace Session Resume**: Automatically scans your current workspace directory (`CWD`), finds the most recent AI conversation session, and resumes it seamlessly.
- **📄 Conversation Markdown Exporter (`export`)**:
  - Run `cli-resumer export` to export uncompressed, full-text conversation transcripts into `AI_SESSION_NOTES.md`!
  - **In-Session Slash Command (`/export`)**: Run `cli-resumer alias` to register `/export` skill inside AGY CLI so you can type `/export` directly inside your AGY CLI session!
- **🎨 Terminal Keyword Highlighting**: Matched search terms are highlighted in bold yellow ANSI colors across selection menus and info cards.
- **⏱️ Relative Time Display**: Displays human-friendly relative time alongside exact timestamps (e.g. `[2 hours ago] [2026-08-07 08:30:25]`).
- **🔍 Keyword Query & Fuzzy Filter (`-q` & Menu Search)**: Search sessions by keyword or type directly inside the interactive menu.
- **🔍 Session Inspection Card (`info`)**: View prompt turns, workspace path, and prompt previews before resuming.
- **⚙️ JSON Config File (`config`)**: Set default CLI targets (`agy`/`copilot`) and selection behaviors in `~/.cli-resumer/config.json`.
- **🔗 Shell Alias Installer (`alias`)**: Auto-install convenient alias commands (`agyr`, `agys`, `cpr`, `cps`) and AGY CLI `/export` skill into PowerShell, Bash, or Zsh.
- **🧹 Empty Session Cleanup (`clean`)**: Safely scan and delete 0-prompt empty session folders with user confirmation.
- **🔄 Built-in Self Auto-Updater (`update`)**: Automatically checks GitHub Releases for new binary releases and updates the binary in-place when running `cli-resumer update`.

---

## 🚀 Installation & Downloads

Download the pre-compiled binary for your operating system directly from [GitHub Releases](https://github.com/BingFengHung/cli-resumer/releases):

- **Windows**: `cli-resumer-windows-amd64.exe`
- **Linux**: `cli-resumer-linux-amd64`
- **macOS (Apple Silicon M1/M2/M3)**: `cli-resumer-macos-arm64`
- **macOS (Intel)**: `cli-resumer-macos-x86_64`

---

## 💡 Usage

### 1. Export Conversation to Markdown
```bash
# Export the latest session transcript in current workspace
cli-resumer export

# Or type inside AGY CLI session window:
/export
```

### 2. Search Session History with Highlighting
```bash
cli-resumer -q rust
```

### 3. Session Inspection Card
```bash
cli-resumer info
```

### 4. Install Shell Aliases & AGY CLI `/export` Skill
```bash
cli-resumer alias
```

### 5. Manage Configuration
```bash
cli-resumer config
```

### 6. Clean Empty Sessions
```bash
cli-resumer clean
```

### 7. Self-Update
```bash
cli-resumer update
```

---

## 📋 Command Line Interface Options

| Subcommand / Flag | Short | Description |
| :--- | :--- | :--- |
| `export` / `--export` | `-e` | Export conversation transcript into `AI_SESSION_NOTES.md` (supports `-o <FILE>`) |
| `alias` | | Install shell aliases (`agyr`, `agys`, `cpr`, `cps`) & AGY CLI `/export` slash command skill |
| `config` | | Launch interactive config wizard (supports `--target`, `--select`, `--edit`) |
| `info` / `--info` | `-i` | Display detailed session inspection card |
| `clean` | | Safely scan and remove empty session directories |
| `--query <KEYWORD>` | `-q` | Search session history by keyword |
| `--select` | `-s` | Display interactive selection menu |
| `--target <TARGET>` | `-t` | Target AI CLI tool: `agy`, `copilot`, `auto` |
| `update` / `--update` | `-u` | Auto-update `cli-resumer` from GitHub Releases |

---

## 📄 License

Distributed under the MIT License.
