# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **語言 Version**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，專為解決每次在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，都需要手動輸入 `/resume` 的痛點。

---

## 🌟 主要功能 (Features)

- **⚡ 自動恢復當前專案對話 (Workspace Auto Resume)**：自動掃描當前專案資料夾 (`CWD`)，找到最近一次的 AI 對話 Session 並自動恢復，完全免去手動輸入 `/resume`。
- **🔍 歷史記憶互動選單 (`--select`)**：提供強大的 CLI 互動選單，列出對話時間、工具來源與 Prompt 主題，讓您精準選擇要載入哪一段歷史記憶。
- **🤖 跨工具支援 (Multi-Provider Support)**：同時支援 Google AGY CLI (`agy`) 與 GitHub Copilot CLI (`copilot` / `gh copilot`)。
- **🔄 一鍵自動檢查更新 (`update`)**：內建自動更新機制，只要執行 `cli-resumer update` 或 `cli-resumer -u`，程式將自動前往 GitHub Releases 檢查並替換為最新版本。
- **☁️ 地端零編譯 & GitHub Actions 自動化**：本機端不需要安裝或執行 `cargo build`。跨平台二進制檔（Windows、Linux、macOS）皆由 GitHub Actions 在雲端自動建置發布至 Releases 頁面供點擊下載。

---

## 🚀 下載與安裝 (Installation)

請至 **[GitHub Releases 頁面](https://github.com/BingFengHung/cli-resumer/releases)** 直接下載對應作業系統的執行檔：

- **Windows**: `cli-resumer-windows-amd64.exe`
- **Linux**: `cli-resumer-linux-amd64`
- **macOS (Apple Silicon M1/M2/M3)**: `cli-resumer-macos-arm64`
- **macOS (Intel)**: `cli-resumer-macos-x86_64`

---

## 💡 使用說明 (Usage)

### 1. 預設模式（自動恢復當前專案最新對話）
```bash
# 自動恢復當前專案目錄下的最新 AGY CLI 對話
cli-resumer

# 自動恢復當前專案目錄下的最新 GitHub Copilot CLI 對話
cli-resumer -t copilot
```

### 2. 互動選單模式（挑選對話記憶）
```bash
# 彈出互動選單選擇當前專案要恢復的 Session
cli-resumer --select

# 搜尋並列出所有專案（全域）的歷史對話紀錄
cli-resumer --select --all-workspaces
```

### 3. 自動更新機制
```bash
# 前往 GitHub Releases 檢查並更新至最新版本
cli-resumer update
# 或使用簡寫
cli-resumer -u
```

---

## 📋 CLI 參數說明 (Command Line Interface Options)

| 參數 / 子指令 | 簡寫 | 說明 |
| :--- | :--- | :--- |
| `update` / `--update` | `-u` | 前往 GitHub Releases 檢查並自動更新至最新版本 |
| `--target <TARGET>` | `-t` | 指定目標 CLI 工具：`agy`（預設）、`copilot`、`auto` |
| `--select` | `-s` | 顯示互動選單手動挑選歷史 Session |
| `--all-workspaces` | `-a` | 搜尋所有專案的對話紀錄（忽略當前目錄過濾） |
| `--id <SESSION_ID>` | | 直接使用指定 Session ID 進行恢復 |
| `--help` | `-h` | 顯示說明訊息 |
| `--version` | `-V` | 顯示版本資訊 |

---

## 🏗️ 技術架構與工作原理

1. **Session 檢測與解析**：讀取各 AI CLI 工具在使用者家目錄下的紀錄檔：
   - AGY CLI：`~/.gemini/antigravity-cli/brain/<session-id>/`
   - Copilot CLI：`~/.copilot-cli/` / `~/.config/github-copilot/`
2. **目錄比對過濾**：解析 JSON/JSONL transcript，比對 Session 的工作目錄與當前 Terminal 目錄。
3. **指令啟動**：傳遞相對應的 `--resume <id>` 參數啟動目標 CLI 工具。
4. **自動更新**：調用 GitHub Releases API (`BingFengHung/cli-resumer/releases/latest`) 檢查最新 Tag 版本，下載新執行檔並原地替換。

---

## 📄 授權條款 (License)

本專案採用 MIT 授權條款。
