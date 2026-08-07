# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **語言 Version**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，專為解決每次在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，都需要手動輸入 `/resume` 的痛點。

---

## 🌟 主要功能 (Features)

- **⚡ 自動恢復當前專案對話 (Workspace Auto Resume)**：自動掃描當前專案資料夾 (`CWD`)，找到最近一次的 AI 對話 Session 並自動恢復。
- **⏱️ 人性化相對時間顯示**：選單中同時顯示相對時間與精確時間（如 `[2 小時前] [2026-08-07 08:30:25]`）。
- **🔍 關鍵字搜尋與即時過濾 (`-q` & 選單打字)**：輸入 `-q <KEYWORD>` 搜尋對話，或在選單開啟時隨時打字進行模糊搜尋。
- **🔍 Session 詳細資訊卡片 (`info`)**：預覽對話輪數、檔案修訂與近 5 次 Prompt 摘要。
- **⚙️ 個人化 JSON 設定檔 (`config`)**：存放於家目錄 `~/.cli-resumer/config.json`，支援設定預設目標工具 (`agy`/`copilot`) 與預設選單行為。
- **🔗 Shell Alias 一鍵安裝 (`alias`)**：自動將快捷指令 (`agyr`, `agys`, `cpr`, `cps`) 寫入 PowerShell / Bash / Zsh 設定檔。
- **🧹 空白對話清理工具 (`clean`)**：自動掃描並安全清理 0 提問的空對話資料夾。
- **🔄 一鍵自動檢查更新 (`update`)**：輸入 `cli-resumer update` 自動檢查並替換至最新 GitHub Release 版本。
- **☁️ 地端零編譯 & GitHub Actions 自動化**：本機端不需要安裝或執行 `cargo build`。二進制檔由 GitHub Actions 在雲端自動交叉編譯與發布。

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
cli-resumer
```

### 2. 關鍵字搜尋對話 (Search / Query)
```bash
cli-resumer -q rust
```

### 3. 查看 Session 詳細卡片 (Info Card)
```bash
cli-resumer info
# 或帶入 -i
cli-resumer -i
```

### 4. 安裝 Shell 快捷別名 (Shell Aliases)
```bash
cli-resumer alias
```

### 5. 清理空白對話 (Clean Empty Sessions)
```bash
cli-resumer clean
```

### 6. 管理設定檔 (Config)
```bash
cli-resumer config
```

### 7. 自動更新 (Self-Update)
```bash
cli-resumer update
```

---

## 📋 CLI 參數與子指令說明 (CLI Commands & Options)

| 子指令 / 參數 | 簡寫 | 說明 |
| :--- | :--- | :--- |
| `info` / `--info` | `-i` | 顯示 Session 詳細資訊卡片 |
| `alias` | | 一鍵安裝 Shell 快捷別名 (`agyr`, `agys`, `cpr`, `cps`) |
| `clean` | | 掃描並安全清理空的對話資料夾（支援 `-y` 跳過確認） |
| `config` | | 檢視與產生 `~/.cli-resumer/config.json` 個人化設定檔 |
| `--query <KEYWORD>` | `-q` | 搜尋過往 Prompt 對話內容或 Session ID |
| `--select` | `-s` | 顯示互動選單手動挑選歷史 Session |
| `--target <TARGET>` | `-t` | 指定目標 CLI 工具：`agy`（預設）、`copilot`、`auto` |
| `--all-workspaces` | `-a` | 搜尋所有專案的對話紀錄（忽略當前目錄過濾） |
| `update` / `--update` | `-u` | 前往 GitHub Releases 檢查並自動更新至最新版本 |

---

## 📄 授權條款 (License)

本專案採用 MIT 授權條款。
