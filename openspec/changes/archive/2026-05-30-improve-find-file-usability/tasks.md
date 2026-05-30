## 1. Default Search Directory

- [x] 1.1 Change `-d` default from `/` to `.` in `cli.rs`
- [x] 1.2 Remove special-case `/` check in `main.rs`
- [x] 1.3 Update CLI test for default directory

## 2. Streaming Output

- [x] 2.1 Add callback parameter to `search_filename` in `search.rs`
- [x] 2.2 Add callback parameter to `search_content` in `search.rs`
- [x] 2.3 Call callback instead of collecting results in search functions
- [x] 2.4 Update `main.rs` to pass printing callback to search
- [x] 2.5 Handle combined search (`-n` + `-c`) with streaming

## 3. NDJSON Format

- [x] 3.1 Change `print_json` in `output.rs` to output NDJSON
- [x] 3.2 Update output tests for NDJSON format
