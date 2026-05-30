## Why

The find-file CLI has three usability issues:
1. Default search root is `/` (machine root), which is slow and overwhelming — users almost always want to search from the current directory
2. Results wait until all files are scanned before outputting, which feels slow for large directories
3. JSON output uses a JSON array, which is hard to stream and process line-by-line

## What Changes

- **BREAKING**: Default search directory changed from `/` to `.` (current working directory)
- Streaming output: results print as they're found, not buffered
- **BREAKING**: JSON output format changed from JSON array `["path1", "path2"]` to NDJSON (one JSON object per line)

## Capabilities

### New Capabilities

- `streaming-output`: Streaming results as they're found during directory traversal
- `ndjson-format`: Newline Delimited JSON output format (each result is a separate JSON object per line)

### Modified Capabilities

- `cli-parsing`: Default for `-d` flag changes from `/` to `.`
- `output-formatting`: JSON output format changes from JSON array to NDJSON

## Impact

- `src/cli.rs`: Change default value for `-d` flag
- `src/main.rs`: Remove buffering, print results as they come
- `src/output.rs`: Implement NDJSON output format
- `src/search.rs`: Yield results during traversal instead of collecting all first
