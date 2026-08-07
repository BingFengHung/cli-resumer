# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Language**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` is a lightweight, cross-platform CLI tool written in **Rust**. It eliminates the hassle of manually entering `/resume` every time you launch AI CLI coding assistants (such as **Google AGY CLI** or **GitHub Copilot CLI**) in your project workspace.

---

## 🌟 Key Features

- **⚡ Automatic Workspace Session Resume**: Automatically scans your current workspace directory (`CWD`), finds the most recent AI conversation session, and resumes it seamlessly.
- **⚙️ Convenient Interactive Config Setup (`config`)**:
  - Run `cli-resumer config` to launch an interactive TUI config setup!
  - Or run `cli-resumer config --target copilot` to switch your default tool in one command.
  - Or run `cli-resumer config --edit` to open `~/.cli-resumer/config.json` in your default text editor.
- **⏱️ Relative Time Display**: Displays human-friendly relative time alongside exact timestamps (e.g. `[2 hours ago] [2026-08-07 08:30:25]`).
- **🔍 Keyword Query & Fuzzy Filter (`-q` & Menu Search)**: Search sessions by keyword or type directly inside the interactive menu.
- **🔍 Session Inspection Card (`info`)**: View prompt turns, workspace path, and prompt previews before resuming.
- **🔗 Shell Alias Installer (`alias`)**: Auto-install convenient alias commands (`agyr`, `agys`, `cpr`, `cps`) into PowerShell, Bash, or Zsh.
- **🧹 Empty Session Cleanup (`clean`)**: Safely scan and delete 0-prompt empty session folders with user confirmation.
- **🔄 Built-in Self Auto-Updater (`update`)**: Automatically checks GitHub Releases for new binary releases and updates the binary in-place when running `cli-resumer update`.

---

## 💡 Usage

### 1. Default Mode
```bash
cli-resumer
```

### 2. Manage Configuration
```bash
# Launch interactive config setup
cli-resumer config

# Set default target to copilot
cli-resumer config --target copilot

# Open config.json in text editor
cli-resumer config --edit
```

### 3. Search Session History
```bash
cli-resumer -q rust
```

### 4. Install Shell Aliases
```bash
cli-resumer alias
```

---

## 📋 Command Line Interface Options

| Subcommand / Flag | Short | Description |
| :--- | :--- | :--- |
| `config` | | Launch interactive config wizard (supports `--target`, `--select`, `--edit`) |
| `info` / `--info` | `-i` | Display detailed session inspection card |
| `alias` | | Install shell aliases (`agyr`, `agys`, `cpr`, `cps`) |
| `clean` | | Safely scan and remove empty session directories |
| `--query <KEYWORD>` | `-q` | Search session history by keyword |
| `--select` | `-s` | Display interactive selection menu |
| `--target <TARGET>` | `-t` | Target AI CLI tool: `agy`, `copilot`, `auto` |
| `update` / `--update` | `-u` | Auto-update `cli-resumer` from GitHub Releases |

---

## 📄 License

Distributed under the MIT License.
