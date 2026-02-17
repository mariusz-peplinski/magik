# Usage Refresh Debugging

If `usage` / `rate limits` refresh appears slow or stuck, use this guide to quickly see where time is being spent.

## Quick Commands

- Refresh active account only: `/limits refresh`
- Refresh all accounts: `/limits refresh-all`
- Start TUI with debug logs enabled: `code --debug`

## Where Logs Are Written

- Default log file: `~/.magik/debug_logs/codex-tui.log`
- If `CODE_HOME` is set, logs are written to `<CODE_HOME>/debug_logs/codex-tui.log`

Tail relevant lines:

```bash
tail -f ~/.magik/debug_logs/codex-tui.log | rg "rate limit refresh|Failed to refresh rate limits"
```

## Expected Refresh Log Phases

When logging is enabled, refresh now emits phase markers:

- `rate limit refresh started`
- `rate limit refresh stream opened`
- `rate limit refresh completed`

Timeout/error markers:

- `rate limit refresh timed out before stream opened`
- `rate limit refresh timed out while waiting for stream events`
- `Failed to refresh rate limits: ...`

## Current Guardrails

The refresh flow has explicit timeouts (45s) for:

- opening the stream
- waiting for stream events / snapshot

This prevents indefinite waiting when upstream is stalled.

