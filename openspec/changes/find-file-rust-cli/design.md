## Context

Building a new Rust CLI tool `find-file` from scratch. The tool simplifies file/folder discovery in Mac/Linux terminals with sensible defaults and a clean interface. Currently no file-finding tool exists in this project.

Target users: developers who want a faster, friendlier alternative to `find` and `grep` for day-to-day file discovery.

## Goals / Non-Goals

**Goals:**
- Fast filename search with glob patterns
- Fast content search in text files only (skip binary)
- JSON and plain text output
- Configurable via TOML config file with sensible defaults
- Self-bootstrapping: first run creates config automatically
- Cross-platform: macOS primary, Linux secondary

**Non-Goals:**
- Replacing `find` for power users (we're not building a full query language)
- Real-time file watching / monitoring
- Search inside compressed archives
- Search inside binary files
- Editing or modifying files — this is read-only

## Decisions

### 1. CLI framework: `clap` with derive macros

**Why**: Ergonomic, type-safe arg parsing. Derive macros reduce boilerplate. Well-maintained, industry standard.

**Alternative considered**: `argh` (lighter) — rejected because clap has better help generation and subcommand support if needed later.

### 2. Directory walking: `walkdir`

**Why**: Fast, handles symlink cycles, respects max depth. Simple API.

**Alternative considered**: `ignore` crate (from ripgrep) — good but pulls in more dependencies. We only need basic walking + depth limit. Can add `ignore` later if `.gitignore` support proves complex.

### 3. Glob matching: `globset` from ripgrep ecosystem

**Why**: Fast, well-tested, supports `*`, `?`, `{}`, `[]` patterns. Same engine ripgrep uses.

**Alternative considered**: `glob` crate — older, less maintained. `regex` for filename — overkill, glob is more natural for filenames.

### 4. Content search: `grep-regex` + `grep-searcher` from ripgrep crates

**Why**: Battle-tested, fast, handles encoding detection, respects size limits. Same engine ripgrep uses internally.

**Alternative considered**: Manual `BufReader` + `regex` — reinventing the wheel, slower, more edge cases.

### 5. Parallelism: `rayon` for parallel directory walking

**Why**: Trivial to add, significant speedup on multi-core machines. Walk directories in parallel, collect results.

**Alternative considered**: `tokio` async — overkill for I/O-bound file walking. Rayon's work-stealing is simpler and fast enough.

### 6. Binary detection: extension list + null byte check

**Why**: Fast (no external process), covers 99% of cases. Extension list for known text files, null byte heuristic as fallback.

**Alternative considered**: `file` command — requires spawning process, slower, not always available on all systems.

### 7. Config path: `$HOME/.config/find-file/config.toml`

**Why**: Follows XDG convention (most Linux tools use this). On macOS, `~/.config` is common for CLI tools.

**Alternative considered**: `$XDG_CONFIG_HOME` — more correct but adds complexity. Using `$HOME/.config` as default is simpler and works on both platforms.

### 8. Config override chain: CLI flag > config > defaults

**Why**: Standard pattern. CLI flags always win. Config provides persistence. Defaults ensure it works out of the box.

## Risks / Trade-offs

- **[Risk] Searching from root `/` is slow** → Mitigation: default to home `~` or CWD, add depth limit flag, skip hidden dirs by default
- **[Risk] Permission errors on system dirs** → Mitigation: silently skip permission-denied dirs, don't crash, show warning count at end
- **[Risk] Large files for content search** → Mitigation: `max_file_size_mb` config, skip files over limit
- **[Trade-off] No `.gitignore` support initially** → Can add later with `ignore` crate if needed
- **[Trade-off] Glob only for filename (no regex)** → Glob covers 95% of use cases, can add regex flag later

## Open Questions

- Should we support multiple search roots (search several directories at once)?
- Should `-n` and `-c` be combinable (AND logic) or mutually exclusive?
