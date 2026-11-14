# 🔍 find-file

> 🎉 The friendliest file finder for your terminal! No more yelling at `find` command! 🙉

---

## 📖 What is this?

`find-file` is a simple, fast, and fun CLI tool to search files and folders on your Mac (and Linux too! 🐧). 

It's like having a really good detective 🔎 in your terminal who actually listens to you!

---

## 🚀 Quick Start

### 📦 Install (macOS — no clone, no Rust)

| Your Mac | Download |
|---|---|
| Apple Silicon (M1/M2/M3/M4) | [`find-file-0.1.0-aarch64-apple-darwin.tar.gz`](https://github.com/nine-ai-2046-1/find-file/releases/download/v0.1.0/find-file-0.1.0-aarch64-apple-darwin.tar.gz) |
| Intel | [`find-file-0.1.0-x86_64-apple-darwin.tar.gz`](https://github.com/nine-ai-2046-1/find-file/releases/download/v0.1.0/find-file-0.1.0-x86_64-apple-darwin.tar.gz) |

Not sure which one? Run `uname -m`: `arm64` = Apple Silicon, `x86_64` = Intel.

All releases: https://github.com/nine-ai-2046-1/find-file/releases

```bash
# Apple Silicon (swap the filename for Intel)
curl -L -o find-file.tar.gz \
  https://github.com/nine-ai-2046-1/find-file/releases/download/v0.1.0/find-file-0.1.0-aarch64-apple-darwin.tar.gz

tar -xzf find-file.tar.gz
cd find-file-0.1.0-aarch64-apple-darwin
./find-file --help
```

Each release also ships a `.sha256` sidecar so you can verify the archive:

```bash
shasum -a 256 -c find-file-0.1.0-aarch64-apple-darwin.tar.gz.sha256
```

**If macOS refuses to open it:** the binary is unsigned, so browser-downloaded copies get a quarantine flag. Clear it (no admin rights needed):

```bash
xattr -d com.apple.quarantine ./find-file
```

Downloading with `curl` avoids the flag entirely. Building from source also produces an unflagged binary.

Optional — put it on your `PATH`:

```bash
mkdir -p ~/bin && mv find-file ~/bin/
```

### 🐧 Linux / Windows

No prebuilt binaries are published for these platforms — build from source:

```bash
git clone https://github.com/nine-ai-2046-1/find-file
cd find-file
cargo build --release --locked
# Binary: target/release/find-file
```

Or with Cargo directly:

```bash
cargo install --path .
```

### Basic Usage

```bash
# Search by filename 🔎
find-file -n "config.toml"

# Search inside files 📝
find-file -c "TODO"

# Both at the same time! 🎯
find-file -n "*.log" -c "error"
```

---

## 🎮 Commands & Options

| Flag | Description | Default |
|------|-------------|---------|
| `-n` | Search by filename (glob pattern) | - |
| `-c` | Search file content (text files only) | - |
| `-t` | Filter by type: `file`, `dir`, or `all` | `all` |
| `-d` | Search directory | `~` (home) |
| `-o` | Output format: `text` or `json` | `text` |
| `--case-sensitive` | Make search case-sensitive | off |

---

## 💡 Examples

### 🔎 Find all TOML files

```bash
find-file -n "*.toml"
```

Output:
```
/Users/you/project/Cargo.toml
/Users/you/project/config.toml
```

### 📝 Search for "TODO" in your code

```bash
find-file -c "TODO" -d ./src
```

### 🎯 Find only directories named "src"

```bash
find-file -n "src" -t dir
```

### 📊 Get JSON output (NDJSON format)

```bash
find-file -n "*.js" -o json
```

Output:
```json
{"path":"/Users/you/project/app.js"}
{"path":"/Users/you/project/utils.js"}
```

### 🔤 Case-sensitive search

```bash
find-file -c "Error" --case-sensitive
```

---

## 🏃 Performance Tips

1. **Search specific directories** - Don't search your whole computer! Use `-d` to narrow it down
   ```bash
   find-file -n "*.rs" -d ./src  # ⚡ Fast!
   find-file -n "*.rs"           # 🐌 Slow (searches entire home)
   ```

2. **Use specific patterns** - `*.txt` is faster than `*`
   ```bash
   find-file -n "*.toml"  # ⚡ Fast!
   find-file -n "*"       # 🐌 Finds everything (why?)
   ```

---

## 🎨 Output Formats

### Plain Text (default)
```
/path/to/file1.txt
/path/to/file2.txt
```

### JSON (NDJSON - one object per line)
```json
{"path":"/path/to/file1.txt"}
{"path":"/path/to/file2.txt"}
```

💡 **Pro tip:** Use with `jq` for fancy JSON processing!
```bash
find-file -n "*.toml" -o json | jq -r '.path'
```

---

## ⚙️ Configuration

First run creates a config file at `~/.config/find-file/config.toml`:

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

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| No results found | Check if file exists! Try `ls -la` first |
| Permission denied | Some directories need `sudo` |
| Too many results | Use `-d` to search a specific folder |
| Slow search | Search in a smaller directory |
| `"cannot be opened"` on macOS | `xattr -d com.apple.quarantine ./find-file` |

---

## 📝 License

MIT © 2026

---

## 🙏 Thanks

- [clap](https://docs.rs/clap) - For parsing arguments
- [walkdir](https://docs.rs/walkdir) - For walking directories
- [globset](https://docs.rs/globset) - For glob matching
- [serde](https://docs.rs/serde) - For JSON magic

---

<p align="center">Made with ❤️ and ☕</p>
