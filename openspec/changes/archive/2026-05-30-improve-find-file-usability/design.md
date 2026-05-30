## Context

The find-file CLI currently buffers all results before printing and defaults to searching from root `/`. This makes it feel slow and requires users to always specify `-d .` for local searches.

## Goals / Non-Goals

**Goals:**
- Default search to current directory (`.`)
- Stream results as they're found (print immediately)
- Use NDJSON format for JSON output (one object per line)

**Non-Goals:**
- Parallel search (rayon) — not needed for streaming
- Changing search logic or patterns
- Adding new CLI flags

## Decisions

### 1. Default `-d` to `.`

**Why**: Users almost always want to search from current directory. Searching from root `/` is slow and produces too many results.

**Change**: `cli.rs` line 20: `default_value = "/"` → `default_value = "."`

### 2. Streaming with callback pattern

**Why**: Simple, minimal changes. Pass a callback to search functions that prints each result immediately.

**Alternative considered**: Channels/iterators — more complex, not needed for single-threaded use.

**Change**: 
- `search.rs`: `search_filename` and `search_content` accept `&mut dyn FnMut(PathBuf)` callback
- Call `callback(abs_path)` instead of `results.push(abs_path)`
- `main.rs`: Create printing callback, pass to search

### 3. NDJSON format

**Why**: Each line is a valid JSON object. Easy to process with `jq`, `grep`, streaming tools. Standard format for line-by-line JSON.

**Format**: `{"path":"/abs/path/to/file"}` per line

**Change**: `output.rs`: `print_json` outputs one object per line instead of array

## Risks / Trade-offs

- **[Trade-off] Combined search `-n` + `-c`**: With streaming, intersection logic changes. We'll collect filename matches first, then filter content matches against them. This means filename results are buffered but content results stream.
- **[Risk] Breaking change for scripts**: JSON array → NDJSON breaks existing `jq` pipelines. Users need to update: `jq .[]` → `jq .` (or use `-r`).
