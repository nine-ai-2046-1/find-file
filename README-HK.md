# 🔍 find-file

> 🎉 你個terminal最友善嘅檔案搜尋器！唔使再對住 `find` 命令發脾氣啦！😤拜拜！

---

## 📖 呢個係咩嚟？

`find-file` 係一個簡單、快速、又好玩嘅CLI工具，幫你喺Mac（同Linux啦！🐧）搵檔案同資料夾。

就好似喺你個terminal請咗個好醒嘅偵探 🔎，而且佢真係會聽你講嘢！

---

## 🚀 快速開始

### 安裝

```bash
cargo install --path .
```

### 基本用法

```bash
# 按名稱搜尋檔案 🔎
find-file -n "config.toml"

# 搜尋檔案入面嘅內容 📝
find-file -c "TODO"

# 兩個一齊用！🎯
find-file -n "*.log" -c "error"
```

---

## 🎮 指令同選項

| 標誌 | 描述 | 預設值 |
|------|------|--------|
| `-n` | 按名稱搜尋（支援萬用字元） | - |
| `-c` | 搜尋檔案內容（只限文字檔案） | - |
| `-t` | 篩選類型：`file`、`dir` 或 `all` | `all` |
| `-d` | 搜尋目錄 | `~`（主目錄） |
| `-o` | 輸出格式：`text` 或 `json` | `text` |
| `--case-sensitive` | 大小寫敏感搜尋 | 閂 |

---

## 💡 例子

### 🔎 搵所有TOML檔案

```bash
find-file -n "*.toml"
```

輸出：
```
/Users/你/project/Cargo.toml
/Users/你/project/config.toml
```

### 📝 搜尋你嘅code入面嘅"TODO"

```bash
find-file -c "TODO" -d ./src
```

### 🎯 搵名叫"src"嘅資料夾

```bash
find-file -n "src" -t dir
```

### 📊 攞JSON輸出（NDJSON格式）

```bash
find-file -n "*.js" -o json
```

輸出：
```json
{"path":"/Users/你/project/app.js"}
{"path":"/Users/你/project/utils.js"}
```

### 🔤 大小寫敏感搜尋

```bash
find-file -c "Error" --case-sensitive
```

---

## 🏃 效能小貼士

1. **搜尋具體嘅目錄** - 唔好搜你成部機！用 `-d` 縮小範圍
   ```bash
   find-file -n "*.rs" -d ./src  # ⚡ 快好多！
   find-file -n "*.rs"           # 🐌 好慢（搜成個home）
   ```

2. **用具體嘅pattern** - `*.txt` 比 `*` 快
   ```bash
   find-file -n "*.toml"  # ⚡ 快！
   find-file -n "*"       # 🐌 搵晒所有嘢（做咩呀？）
   ```

---

## 🎨 輸出格式

### 純文字（預設）
```
/path/to/file1.txt
/path/to/file2.txt
```

### JSON（NDJSON - 每行一個JSON物件）
```json
{"path":"/path/to/file1.txt"}
{"path":"/path/to/file2.txt"}
```

💡 **貼士：** 配合 `jq` 用可以整花臣！
```bash
find-file -n "*.toml" -o json | jq -r '.path'
```

---

## ⚙️ 設定

第一次執行會喺 `~/.config/find-file/config.toml` 建立設定檔：

```toml
search_root = "~"
type = "all"
output = "text"

[content]
text_only = true
case_sensitive = false
max_file_size_mb = 10

[name]
pattern_type = "glob"
case_sensitive = false

[performance]
max_depth = 0
skip_hidden = true
```

---

## 🐛 有問題？

| 問題 | 解決方法 |
|------|----------|
| 搵唔到嘢 | 檢查吓個檔案係咪真係存在！試吓 `ls -la` |
| 權限不足 | 有啲目錄要用 `sudo` |
| 結果太多 | 用 `-d` 搜尋具體嘅資料夾 |
| 搜尋太慢 | 搜細啲嘅目錄 |

---

## 📝 授權條款

MIT © 2026

---

## 🙏 多謝

- [clap](https://docs.rs/clap) - 解析命令行參數
- [walkdir](https://docs.rs/walkdir) - 遍歷目錄
- [globset](https://docs.rs/globset) - 萬用字元配對
- [serde](https://docs.rs/serde) - JSON魔法

---

<p align="center">用 ❤️ 同 ☕ 整嘅</p>
