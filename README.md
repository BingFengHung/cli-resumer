# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Language**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` is a lightweight, cross-platform CLI tool written in **Rust**. It eliminates the hassle of manually entering `/resume` every time you launch AI CLI coding assistants (such as **Google AGY CLI** or **GitHub Copilot CLI**) in your project workspace.

---

## 🌟 Key Features

- **⚡ Automatic Workspace Session Resume**: Automatically scans your current workspace directory (`CWD`), finds the most recent AI conversation session, and resumes it seamlessly.
- **⏱️ Relative Time Display**: Displays human-friendly relative time alongside exact timestamps (e.g. `[2 hours ago] [2026-08-07 08:30:25]`).
- **🔍 Keyword Query & Fuzzy Filter (`-q` & Menu Search)**: Search sessions by keyword or type directly inside the interactive menu.
- **🔍 Session Inspection Card (`info`)**: View prompt turns, workspace path, and prompt previews before resuming.
- **⚙️ JSON Config File (`config`)**: Set default CLI targets (`agy`/`copilot`) and selection behaviors in `~/.config/cli-resumer/config.json`.
- **🔗 Shell Alias Installer (`alias`)**: Auto-install convenient alias commands (`agyr`, `agys`, `cpr`, `cps`) into PowerShell, Bash, or Zsh.
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

### 1. Default Mode
```bash
cli-resumer
```

### 2. Search Session History
```bash
cli-resumer -q rust
```

### 3. Session Inspection Card
```bash
cli-resumer info
```

### 4. Install Shell Aliases
```bash
cli-resumer alias
```

### 5. Clean Empty Sessions
```bash
cli-resumer clean
```

---

## 📋 Command Line Interface Options

| Subcommand / Flag | Short | Description |
| :--- | :--- | :--- |
| `info` / `--info` | `-i` | Display detailed session inspection card |
| `alias` | | Install shell aliases (`agyr`, `agys`, `cpr`, `cps`) |
| `clean` | | Safely scan and remove empty session directories |
| `config` | | View or generate `config.json` configuration file |
| `--query <KEYWORD>` | `-q` | Search session history by keyword |
| `--select` | `-s` | Display interactive selection menu |
| `--target <TARGET>` | `-t` | Target AI CLI tool: `agy` (default), `copilot`, `auto` |
| `update` / `--update` | `-u` | Auto-update `cli-resumer` from GitHub Releases |

---

## 📄 License

Distributed under the MIT License.
