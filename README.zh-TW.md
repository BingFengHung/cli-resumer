# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **語言 Version**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，專為解決每次在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，都需要手動輸入 `/resume` 的痛點。

---

## 🌟 主要功能 (Features)

- **⚡ 自動恢復當前專案對話 (Workspace Auto Resume)**：自動掃描當前專案資料夾 (`CWD`)，找到最近一次的 AI 對話 Session 並自動恢復。
- **⚙️ 便捷設定檔與互動配置 (`config`)**：
  - 直接執行 `cli-resumer config` 開啟互動式設定選單！
  - 亦可執行 `cli-resumer config --target copilot` 一鍵切換預設工具。
  - 或執行 `cli-resumer config --edit` 直接在記事本 / 編輯器中開啟 `~/.cli-resumer/config.json`。
- **⏱️ 人性化相對時間顯示**：選單中同時顯示相對時間與精確時間（如 `[2 小時前] [2026-08-07 08:30:25]`）。
- **🔍 關鍵字搜尋與即時過濾 (`-q` & 選單打字)**：輸入 `-q <KEYWORD>` 搜尋對話，或在選單開啟時隨時打字進行模糊搜尋。
- **🔍 Session 詳細資訊卡片 (`info`)**：預覽對話輪數、檔案修訂與近 5 次 Prompt 摘要。
- **🔗 Shell Alias 一鍵安裝 (`alias`)**：自動將快捷指令 (`agyr`, `agys`, `cpr`, `cps`) 寫入 PowerShell / Bash / Zsh 設定檔。
- **🧹 空白對話清理工具 (`clean`)**：自動掃描並安全清理 0 提問的空對話資料夾。
- **🔄 一鍵自動檢查更新 (`update`)**：輸入 `cli-resumer update` 自動檢查並替換至最新 GitHub Release 版本。

---

## 💡 使用說明 (Usage)

### 1. 預設模式（自動恢復當前專案最新對話）
```bash
cli-resumer
```

### 2. 調整設定檔 (Config)
```bash
# 開啟互動式設定嚮導（切換預設工具、預設選單模式等）
cli-resumer config

# 一鍵直接將預設目標改為 Copilot CLI
cli-resumer config --target copilot

# 在預設文字編輯器中開啟 ~/.cli-resumer/config.json
cli-resumer config --edit
```

### 3. 關鍵字搜尋對話 (Search / Query)
```bash
cli-resumer -q rust
```

### 4. 安裝 Shell 快捷別名 (Shell Aliases)
```bash
cli-resumer alias
```

---

## 📋 CLI 參數與子指令說明 (CLI Commands & Options)

| 子指令 / 參數 | 簡寫 | 說明 |
| :--- | :--- | :--- |
| `config` | | 開啟互動式設定嚮導（支援 `--target`, `--select`, `--edit`） |
| `info` / `--info` | `-i` | 顯示 Session 詳細資訊卡片 |
| `alias` | | 一鍵安裝 Shell 快捷別名 (`agyr`, `agys`, `cpr`, `cps`) |
| `clean` | | 掃描並安全清理空的對話資料夾（支援 `-y` 跳過確認） |
| `--query <KEYWORD>` | `-q` | 搜尋過往 Prompt 對話內容或 Session ID |
| `--select` | `-s` | 顯示互動選單手動挑選歷史 Session |
| `--target <TARGET>` | `-t` | 指定目標 CLI 工具：`agy`、`copilot`、`auto` |
| `update` / `--update` | `-u` | 前往 GitHub Releases 檢查並自動更新至最新版本 |

---

## 📄 授權條款 (License)

本專案採用 MIT 授權條款。
