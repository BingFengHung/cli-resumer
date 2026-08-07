# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **語言 Version**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，專為解決每次在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，都需要手動輸入 `/resume` 的痛點。

---

## 🌟 主要功能 (Features)

- **⚡ 自動恢復當前專案對話 (Workspace Auto Resume)**：自動掃描當前專案資料夾 (`CWD`)，找到最近一次的 AI 對話 Session 並自動恢復。
- **🎨 搜尋關鍵字終端機黃色高亮 (Keyword Highlighting)**：當使用 `-q <KEYWORD>` 搜尋對話或檢視卡片時，匹配到的關鍵字會以亮黃色高亮顯示，搜尋視覺一目了然！
- **⏱️ 人性化相對時間顯示**：選單中同時顯示相對時間與精確時間（如 `[2 小時前] [2026-08-07 08:30:25]`）。
- **🔍 關鍵字搜尋與即時過濾 (`-q` & 選單打字)**：輸入 `-q <KEYWORD>` 搜尋對話，或在選單開啟時隨時打字進行模糊搜尋。
- **🔍 Session 詳細資訊卡片 (`info`)**：預覽對話輪數、檔案修訂與近 5 次 Prompt 摘要。
- **⚙️ 個人化 JSON 設定檔 (`config`)**：存放於家目錄 `~/.cli-resumer/config.json`，支援設定預設目標工具 (`agy`/`copilot`) 與預設選單行為。
- **🔗 Shell Alias 一鍵安裝 (`alias`)**：自動將快捷指令 (`agyr`, `agys`, `cpr`, `cps`) 寫入 PowerShell / Bash / Zsh 設定檔。
- **🧹 空白對話清理工具 (`clean`)**：自動掃描並安全清理 0 提問的空對話資料夾。
- **🔄 一鍵自動檢查更新 (`update`)**：輸入 `cli-resumer update` 自動檢查並替換至最新 GitHub Release 版本。

---

## 💡 使用說明 (Usage)

### 1. 關鍵字搜尋與黃色高亮 (Search with Highlighting)
```bash
# 搜尋包含 "rust" 的對話（匹配關鍵字自動黃色高亮顯示）
cli-resumer -q rust
```

### 2. 查看 Session 詳細卡片 (Info Card)
```bash
cli-resumer info
# 或搭配搜尋高亮
cli-resumer info -q rust
```

---

## 📄 授權條款 (License)

本專案採用 MIT 授權條款。
