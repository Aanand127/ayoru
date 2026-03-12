# Default TUI Command Design

**Goal:** Make bare `ayoru` open the TUI while preserving `ayoru "<query>"` as the direct CLI playback flow.

## Decision

Use argument parsing as the source of truth for command selection.

- `ayoru` -> `Command::Tui`
- `ayoru tui` -> `Command::Tui`
- `ayoru "<query>"` -> `Command::Play`

## Why

- keeps dispatch logic explicit in one place
- preserves the current query-based CLI behavior
- avoids special runtime branching in `main`
- lets tests verify command selection directly

## Error Handling

If the TUI cannot start, keep returning the existing user-facing runtime error from the TUI path. Do not silently fall back to another mode.
