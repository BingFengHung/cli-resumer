# cli-resumer 🚀

[![CI Workflow](https://github.com/BingFengHung/cli-resumer/actions/workflows/ci.yml/badge.svg)](https://github.com/BingFengHung/cli-resumer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **語言 Version**: [English](README.md) | [繁體中文](README.zh-TW.md)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，專為解決每次在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，都需要手動輸入 `/resume` 的痛點。

---

## 🌟 主要功能 (Features)

- **⚡ 自動恢復當前專案對話 (Workspace Auto Resume)**：自動掃描當前專案資料夾 (`CWD`)，找到最近一次的 AI 對話 Session 並自動恢復。
- **📄 對話紀錄一鍵導出 Markdown (`export`)**：
  - 終端機執行 `cli-resumer export` 直接將整段未經壓縮的完整對話導出為排版漂亮的 `AI_SESSION_NOTES.md` 文件！
  - **AGY CLI 內建 Slash Command (`/export`)**：執行 `cli-resumer alias` 自動為 AGY CLI 註冊斜線指令，在 AGY CLI 對話視窗中直接輸入 `/export` 即可秒速產生專案對話文件！
- **🎨 搜尋關鍵字終端機黃色高亮 (Keyword Highlighting)**：當使用 `-q <KEYWORD>` 搜尋對話或檢視卡片時，匹配到的關鍵字會以亮黃色高亮顯示。
- **⏱️ 人性化相對時間顯示**：選單中同時顯示相對時間與精確時間（如 `[2 小時前] [2026-08-07 08:30:25]`）。
- **🔍 關鍵字搜尋與即時過濾 (`-q` & 選單打字)**：輸入 `-q <KEYWORD>` 搜尋對話，或在選單開啟時隨時打字進行模糊搜尋。
- **🔍 Session 詳細資訊卡片 (`info`)**：預覽對話輪數、檔案修訂與近 5 次 Prompt 摘要。
- **⚙️ 個人化 JSON 設定檔 (`config`)**：存放於家目錄 `~/.cli-resumer/config.json`，支援設定預設目標工具 (`agy`/`copilot`) 與預設選單行為。
- **🔗 Shell Alias 一鍵安裝 (`alias`)**：自動將快捷指令 (`agyr`, `agys`, `cpr`, `cps`) 與 AGY CLI `/export` 技能寫入設定檔。
- **🧹 空白對話清理工具 (`clean`)**：自動掃描並安全清理 0 提問的空對話資料夾。
- **🔄 一鍵自動檢查更新 (`update`)**：輸入 `cli-resumer update` 自動檢查並替換至最新 GitHub Release 版本。

---

## 💡 使用說明 (Usage)

### 1. 導出對話為 Markdown (Export Session Notes)
```bash
# 在 Terminal 導出當前專案最近一次對話
cli-resumer export

# 或在 AGY CLI 視窗內直接輸入:
/export
```

### 2. 關鍵字搜尋與黃色高亮 (Search with Highlighting)
```bash
cli-resumer -q rust
```

### 3. 查看 Session 詳細卡片 (Info Card)
```bash
cli-resumer info
```

### 4. 安裝 Shell 快捷別名與 AGY CLI `/export` 技能
```bash
cli-resumer alias
```

---

## 📋 CLI 參數與子指令說明 (CLI Commands & Options)

| 子指令 / 參數 | 簡寫 | 說明 |
| :--- | :--- | :--- |
| `export` / `--export` | `-e` | 將對話逐字紀錄導出為 `AI_SESSION_NOTES.md` (支援 `-o <FILE>`) |
| `alias` | | 一鍵安裝 Shell 快捷別名與 AGY CLI `/export` 斜線指令技能 |
| `config` | | 開啟互動式設定嚮導（支援 `--target`, `--select`, `--edit`） |
| `info` / `--info` | `-i` | 顯示 Session 詳細資訊卡片 |
| `clean` | | 掃描並安全清理空的對話資料夾（支援 `-y` 跳過確認） |
| `--query <KEYWORD>` | `-q` | 搜尋過往 Prompt 對話內容或 Session ID |
| `--select` | `-s` | 顯示互動選單手動挑選歷史 Session |
| `--target <TARGET>` | `-t` | 指定目標 CLI 工具：`agy`、`copilot`、`auto` |
| `update` / `--update` | `-u` | 前往 GitHub Releases 檢查並自動更新至最新版本 |

---

## 📄 授權條款 (License)

本專案採用 MIT 授權條款。
