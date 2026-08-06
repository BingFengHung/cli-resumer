# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **語言 Version**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，專為解決每次在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，都需要手動輸入 `/resume` 的痛點。

---

## 🌟 主要功能 (Features)

- **⚡ 自動恢復當前專案對話 (Workspace Auto Resume)**：自動掃描當前專案資料夾 (`CWD`)，找到最近一次的 AI 對話 Session 並自動恢復，完全免去手動輸入 `/resume`。
- **🔍 關鍵字搜尋與互動選單 (`-q` & `--select`)**：
  - **命令列搜尋 (`-q <KEYWORD>`)**：直接在命令列輸入關鍵字搜尋過往對話 Prompt 或 Session ID。
  - **即時模糊搜尋選單**：在選單開啟時，可隨時直接打字過濾對話選單。
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

### 2. 關鍵字搜尋對話 (Search / Query)
```bash
# 搜尋包含 "rust" 關鍵字的對話紀錄
cli-resumer -q rust

# 搜尋全域（跨專案）包含 "ios" 關鍵字的對話
cli-resumer -q ios -a
```

### 3. 互動選單模式 (Interactive Menu)
```bash
# 彈出互動選單選擇當前專案要恢復的 Session（可在選單中直接打字搜尋）
cli-resumer --select

# 搜尋並列出所有專案（全域）的歷史對話紀錄
cli-resumer --select --all-workspaces
```

### 4. 自動更新機制
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
| `--query <KEYWORD>` | `-q` | 搜尋過往 Prompt 對話內容或 Session ID |
| `--select` | `-s` | 顯示互動選單手動挑選歷史 Session（選單內亦可即時打字搜尋） |
| `--target <TARGET>` | `-t` | 指定目標 CLI 工具：`agy`（預設）、`copilot`、`auto` |
| `--all-workspaces` | `-a` | 搜尋所有專案的對話紀錄（忽略當前目錄過濾） |
| `update` / `--update` | `-u` | 前往 GitHub Releases 檢查並自動更新至最新版本 |
| `--id <SESSION_ID>` | | 直接使用指定 Session ID 進行恢復 |
| `--help` | `-h` | 顯示說明訊息 |
| `--version` | `-V` | 顯示版本資訊 |

---

## 📄 授權條款 (License)

本專案採用 MIT 授權條款。
