# Ayoru

`ayoru` is a quieter way to watch anime.

Ayoru is a Rust CLI and terminal UI for searching anime, picking episodes, and launching playback with a cleaner, calmer flow than the usual script-and-site experience.

## What It Does

- `ayoru <query>` runs the direct CLI flow
- `ayoru tui` opens the dashboard-style TUI shell
- searches AllAnime
- lets you choose titles and episodes
- ranks streams with a deterministic provider preference
- detects a local player and launches playback
- stores local TUI data for:
  - favorites
  - recently watched
  - history

## Requirements

- Rust stable toolchain
- one supported player installed:
  - `mpv`
  - `iina`
  - `vlc`

## Install

### Run from source

```bash
git clone <your-remote-url>
cd ayoru
cargo run -- "frieren"
```

### Install locally with Cargo

```bash
cargo install --path .
```

Then use:

```bash
ayoru "frieren"
ayoru tui
```

## Quick Start

### CLI flow

```bash
ayoru "frieren"
```

This:
1. searches for the title
2. lets you choose a result
3. lets you choose an episode
4. resolves streams
5. opens playback in your local player

### TUI shell

```bash
ayoru tui
```

The TUI gives you:
- a persistent shell layout
- search
- favorites
- recently watched
- history
- keyboard-first navigation

## TUI Controls

- type to search when search is focused
- `/` focus search
- `Tab` move between shell panels
- `h` / `l` move panel focus when search is not focused
- `j` / `k` or arrow keys move inside the active panel
- `Enter` confirm/select in the active main flow
- `f` toggle favorite when search is not focused
- `Esc` back out of detail/playback states
- `q` quit

## Local Data

The TUI stores local state at:

- `$XDG_STATE_HOME/ayoru/library.json`, or
- `~/.local/state/ayoru/library.json`

This file contains favorites, recently watched, and history in a simple JSON format.

## Current Scope

Ayoru currently focuses on:
- terminal-first playback flow
- local-only saved state
- one provider integration
- local player launch

It does not include:
- sync
- accounts
- recommendations
- collections
- a native desktop app yet

## Repo Layout

```text
src/
  app.rs          app orchestration and runtime traits
  args.rs         command parsing
  cli/            legacy prompt-style picker flow
  core/           models, playback policy, stream ranking
  player/         player detection and launch
  provider/       provider integrations
  tui/            dashboard shell, renderer, runtime, persistence
tests/            integration and behavior tests
docs/             design and implementation docs
```

## Development

Useful commands:

```bash
cargo run -- --help
cargo test -q
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Notes

- The TUI shell and the direct CLI share the same provider/playback stack.
- Favorites, history, and recently watched are local-only by design right now.
- The repo includes design and implementation docs under `docs/plans/` for the TUI, branding, and shell work.
