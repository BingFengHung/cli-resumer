# AGENTS.md - Developer & Agent Guidelines

本文件供 AI Agent 及開發者參考，規範 `cli-resumer` 專案的開發原則、版本管控、GitHub Actions CI/CD 工作流及程式碼維護標準。

---

## 🎯 專案簡介與目標 (Project Overview)

`cli-resumer` 是一個使用 **Rust** 開發的高效能跨平台 CLI 工具，旨在使用戶在專案目錄開啟 AI CLI 輔助工具（如 **Google AGY CLI** 與 **GitHub Copilot CLI**）時，能自動恢復當前專案最新一次的對話紀錄，免去手動輸入 `/resume`。

---

## ⚠️ 核心開發與規範原則 (Core Guidelines & Rules)

### 1. 🚫 地端零編譯原則 (Zero Local Compilation)
- **絕對不要在地端執行 `cargo build` 或編譯產出二進制檔**。
- 地端僅進行程式碼編寫、文件修訂與 Git 版本控制。
- 所有 Rust 二進制檔（Windows `.exe`、Linux、macOS）的交叉編譯**必須完全交由 GitHub Actions 雲端 CI/CD 執行**。

### 2. 🔢 版本號規範 (Versioning & Release Tags)
- **每次更新或修改程式碼功能時，必須同時升級版本號**。
- 升級步驟：
  1. 更新 `Cargo.toml` 中的 `version` 欄位（例如 `0.2.0` -> `0.3.0`）。
  2. 完成 Git 提交與推送主分支 (`main`)。
  3. 建立相對應的版本 Tag 並推送至 GitHub（例如 `git tag v0.3.0` -> `git push origin v0.3.0`）。

### 3. 💬 Commit Message 規範 (Commit Message Rules)
- **所有 Git Commit Message 必須使用繁體中文撰寫**。
- 訊息格式應清晰說明異動動機與變更內容（例如：`新增自動更新功能 (update 指令)、升級版本至 v0.2.0`）。

### 4. 📚 雙語文件維護 (Bilingual Documentation)
- 保持雙語說明文件同步更新：
  - `README.md`（英文版）
  - `README.zh-TW.md`（繁體中文版）
- 兩份文件需相互包含語言切換連結。

---

## 📁 專案架構與檔案說明 (Project Architecture)

```
cli-resumer/
├── Cargo.toml               # 專案依賴與版本設定
├── AGENTS.md                # AI Agent 與開發規範文件
├── README.md                # 英文說明文件
├── README.zh-TW.md          # 繁體中文說明文件
├── .gitignore               # Git 忽略設定
├── .github/
│   └── workflows/
│       ├── ci.yml           # CI 矩陣檢查工作流 (cargo check)
│       └── release.yml      # Release 自動交叉編譯與 GitHub Release 發佈工作流
└── src/
    ├── main.rs              # CLI 命令解析 (clap) 與主控流程
    ├── session.rs           # Session 資訊結構與時間格式化
    ├── ui.rs                # 互動式選單 (inquire)
    ├── updater.rs           # 透過 self_update 實現一鍵自動更新
    └── providers/
        ├── mod.rs
        ├── agy.rs           # Google AGY CLI 歷史紀錄讀取與恢復
        └── copilot.rs       # GitHub Copilot CLI 歷史紀錄讀取與恢復
```

---

## 🔄 CLI 指令與自動更新機制 (Self-Updater)

- **預設自動恢復**：`cli-resumer`（AGY CLI） / `cli-resumer -t copilot`（Copilot CLI）。
- **互動選單**：`cli-resumer --select` / `-s`。
- **自動更新指令**：`cli-resumer update` 或 `cli-resumer -u`。
  - 自動調用 GitHub Releases API (`BingFengHung/cli-resumer/releases/latest`) 檢查最新 Tag 版本，並自動下載對應平台二進制檔進原地替換。

---

## 🚀 工作流程與發布步驟 (Release Workflow)

當進行程式碼修改與功能更新時，請遵循以下標準流程：

```bash
# 1. 編輯 Cargo.toml 升級版本號，並更新對應程式碼與雙語 README

# 2. 進行 Git 提交 (使用全中文 commit message)
git add .
git commit -m "更新說明與新功能描述..."

# 3. 推送主分支
git push origin main

# 4. 建立對應版本的 Release Tag 並推送（觸發 GitHub Actions 雲端編譯）
git tag vX.Y.Z
git push origin vX.Y.Z
```
