# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Language**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` is a lightweight, cross-platform CLI tool written in **Rust**. It eliminates the hassle of manually entering `/resume` every time you launch AI CLI coding assistants (such as **Google AGY CLI** or **GitHub Copilot CLI**) in your project workspace.

---

## 🌟 Key Features

- **⚡ Automatic Workspace Session Resume**: Automatically scans your current workspace directory (`CWD`), finds the most recent AI conversation session, and resumes it seamlessly.
- **🔍 Keyword Query & Real-time Fuzzy Search (`-q` & `--select`)**:
  - **CLI Query Flag (`-q <KEYWORD>`)**: Search session history directly by prompt content or session ID via command line.
  - **In-Menu Real-Time Filter**: Start typing anytime inside the interactive menu to instantly filter conversation items.
- **🤖 Multi-Provider Support**: Seamlessly supports both Google AGY CLI (`agy`) and GitHub Copilot CLI (`copilot` / `gh copilot`).
- **🔄 Built-in Self Auto-Updater (`update`)**: Automatically checks GitHub Releases for new binary releases and updates the binary in-place when running `cli-resumer update` or `cli-resumer -u`.
- **☁️ Zero-Local-Compilation CI/CD**: Fully automated cross-platform builds via GitHub Actions. Pre-compiled binaries for Windows, Linux, and macOS are automatically generated and published directly on GitHub Releases.

---

## 🚀 Installation & Downloads

Download the pre-compiled binary for your operating system directly from [GitHub Releases](https://github.com/BingFengHung/cli-resumer/releases):

- **Windows**: `cli-resumer-windows-amd64.exe`
- **Linux**: `cli-resumer-linux-amd64`
- **macOS (Apple Silicon M1/M2/M3)**: `cli-resumer-macos-arm64`
- **macOS (Intel)**: `cli-resumer-macos-x86_64`

---

## 💡 Usage

### 1. Default Mode (Auto-Resume Latest Workspace Session)
```bash
# Auto-resumes the latest AGY CLI session in current workspace
cli-resumer

# Auto-resumes the latest GitHub Copilot CLI session
cli-resumer -t copilot
```

### 2. Search Session History (Query Flag)
```bash
# Search sessions matching "rust"
cli-resumer -q rust

# Search sessions across all workspaces matching "ios"
cli-resumer -q ios -a
```

### 3. Interactive Selection Mode
```bash
# Opens an interactive menu (you can type keywords directly to filter)
cli-resumer --select

# Searches and lists sessions across all workspaces
cli-resumer --select --all-workspaces
```

### 4. Self-Updating
```bash
# Automatically checks GitHub Releases and updates cli-resumer to the latest release
cli-resumer update
# OR
cli-resumer -u
```

---

## 📋 Command Line Interface Options

| Flag / Subcommand | Short | Description |
| :--- | :--- | :--- |
| `--query <KEYWORD>` | `-q` | Search session history by keyword in prompt text or session ID |
| `--select` | `-s` | Display interactive selection menu (supports real-time typing filter) |
| `--target <TARGET>` | `-t` | Target AI CLI tool: `agy` (default), `copilot`, `auto` |
| `--all-workspaces` | `-a` | Search session history across all directories (ignore CWD filter) |
| `update` / `--update` | `-u` | Check GitHub Releases and auto-update `cli-resumer` to the latest version |
| `--id <SESSION_ID>` | | Resume a specific session directly by ID |
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version information |

---

## 📄 License

Distributed under the MIT License.
