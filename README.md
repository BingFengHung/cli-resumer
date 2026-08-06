# cli-resumer 🚀

`cli-resumer` 是一個使用 **Rust** 開發的 CLI 工具，專門為解決在專案資料夾啟動 AI CLI 工具（如 **AGY CLI** 與 **GitHub Copilot CLI**）時，每次都需要手動輸入 `/resume` 的痛點。

---

## ✨ 主要功能 (Features)

1. **自動恢復最新對話 (Auto Resume)**：
   - 預設自動讀取當前專案資料夾（Workspace）中最近一次的 AI 對話 Session，免去手動輸入 `/resume`。
2. **對話記憶選擇選單 (Interactive Session Selector)**：
   - 使用 `-s` 或 `--select` 參數時，呈現互動式選單，列出過往歷史對話時間與主題簡述，隨心選擇要載入哪一段記憶。
3. **跨工具支援 (Multi-Provider Support)**：
   - 同時支援 Google AGY CLI (`agy`) 與 GitHub Copilot CLI (`copilot` / `gh copilot`)。
4. **雲端 GitHub Actions 自動編譯 (Zero Local Compilation)**：
   - 地端不需要安裝或執行 `cargo build`，所有二進制檔 (Windows `.exe`, Linux, macOS) 皆由 GitHub Actions 自動矩陣編譯發布！

---

## 🛠️ 使用方式 (Usage)

### 1. 預設模式（自動恢復當前專案最新對話）
```bash
# 自動恢復當前專案目錄下的最新 AGY CLI 對話
cli-resumer

# 指定 GitHub Copilot CLI
cli-resumer -t copilot
```

### 2. 互動式選擇歷史對話
```bash
# 彈出互動選單選擇要恢復的 Session
cli-resumer --select

# 選擇全域（所有專案）的歷史對話
cli-resumer --select --all-workspaces
```

### 3. CLI 參數說明 (Command Line Options)
| 參數 | 簡寫 | 說明 |
| :--- | :--- | :--- |
| `--target <TARGET>` | `-t` | 指定目標 CLI 工具：`agy`（預設）、`copilot`、`auto` |
| `--select` | `-s` | 顯示互動選單手動挑選 Session |
| `--all-workspaces` | `-a` | 搜尋所有專案的對話紀錄（預設僅比對當前工作目錄） |
| `--id <SESSION_ID>` | | 直接指定 Session ID 進行恢復 |

---

## ⚙️ GitHub Actions 自動編譯工作流 (CI/CD Workflows)

所有的 Rust 程式碼編譯全部交由 `.github/workflows/release.yml` 執行：
- **Windows (x86_64-pc-windows-msvc)** -> `cli-resumer-windows-amd64.exe`
- **Linux (x86_64-unknown-linux-gnu)** -> `cli-resumer-linux-amd64`
- **macOS (x86_64 & aarch64)** -> `cli-resumer-macos`

當推送到 `main` 分支或建立 Tag 時，GitHub Actions 將會自動產出對應平台的 Executable 供下載使用。
