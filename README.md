# cli-resumer 🚀

[![CI Workflow](https://github.com/user/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/user/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Language**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` is a lightweight, cross-platform CLI tool written in **Rust**. It eliminates the hassle of manually entering `/resume` every time you launch AI CLI coding assistants (such as **Google AGY CLI** or **GitHub Copilot CLI**) in your project workspace.

---

## 🌟 Key Features

- **⚡ Automatic Workspace Session Resume**: Automatically scans your current workspace directory (`CWD`), finds the most recent AI conversation session, and resumes it seamlessly.
- **🔍 Interactive Memory Selector (`--select`)**: Offers an interactive fuzzy-search TUI menu listing session timestamps, providers, and prompt topics—allowing you to pick exactly which session memory to restore.
- **🤖 Multi-Provider Support**: Seamlessly supports both Google AGY CLI (`agy`) and GitHub Copilot CLI (`copilot` / `gh copilot`).
- **☁️ Zero-Local-Compilation CI/CD**: Fully automated cross-platform builds via GitHub Actions. Pre-compiled binaries for Windows, Linux, and macOS are automatically generated on releases.

---

## 🚀 Installation & Downloads

Download the pre-compiled binary for your operating system from [GitHub Releases](https://github.com/user/cli-resumer/releases):

- **Windows**: `cli-resumer-windows-amd64.exe`
- **Linux**: `cli-resumer-linux-amd64`
- **macOS (Apple Silicon)**: `cli-resumer-macos-arm64`
- **macOS (Intel)**: `cli-resumer-macos-x86_64`

---

## 💡 Usage

### 1. Default Mode (Auto-Resume Latest Workspace Session)
```bash
# Auto-resumes the latest AGY CLI session in the current project directory
cli-resumer

# Auto-resumes the latest GitHub Copilot CLI session
cli-resumer -t copilot
```

### 2. Interactive Selection Mode
```bash
# Opens an interactive menu to choose a session for the current workspace
cli-resumer --select

# Searches and lists sessions across all workspaces
cli-resumer --select --all-workspaces
```

---

## 📋 Command Line Interface Options

| Flag | Short | Description |
| :--- | :--- | :--- |
| `--target <TARGET>` | `-t` | Target AI CLI tool: `agy` (default), `copilot`, `auto` |
| `--select` | `-s` | Display interactive selection menu to choose a session |
| `--all-workspaces` | `-a` | Search session history across all directories (ignore CWD filter) |
| `--id <SESSION_ID>` | | Resume a specific session directly by ID |
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version information |

---

## 🏗️ Architecture & How It Works

1. **Session Detection**: Scans provider storage paths:
   - AGY CLI: `~/.gemini/antigravity-cli/brain/<session-id>/`
   - Copilot CLI: `~/.copilot-cli/` / `~/.config/github-copilot/`
2. **Workspace Filtering**: Parses JSON/JSONL transcripts to match session working directories against your current directory.
3. **Execution**: Launches the target CLI with the corresponding `--resume <id>` or `resume <id>` command.

---

## ⚙️ GitHub Actions Matrix Build

All releases are compiled automatically via GitHub Actions workflows ([`.github/workflows/release.yml`](.github/workflows/release.yml)). No local Rust compilation required!

---

## 📄 License

Distributed under the MIT License.
