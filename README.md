# Ayoru

`ayoru` is a terminal-first anime watcher with a direct CLI flow and a keyboard-driven TUI.

It lets you search a show, pick an episode, resolve a stream, and open playback in a local player without bouncing between browser tabs and ad-heavy sites.

## Why Use It

- bare `ayoru` opens the full-screen TUI
- fast CLI flow for `ayoru "<show>"`
- full-screen TUI with search, favorites, history, and recently watched
- automatic stream ranking and playback fallback
- local player launch with `mpv`, `iina`, or `vlc`
- local-only saved state for the TUI

## Requirements

Before installing, make sure you have:

- Rust stable toolchain
- one supported player installed:
  - `mpv`
  - `iina`
  - `vlc`

## Install

### One-command install

```bash
curl -fsSL https://raw.githubusercontent.com/BrandonLee28/ayoru/main/scripts/install.sh | sh
```

What this does:

- installs `ayoru` to `~/.local/bin/ayoru`
- uses a GitHub release artifact when available
- falls back to a source build if a release artifact is missing
- adds `~/.local/bin` to your shell `PATH` if needed

Verify the install:

```bash
command -v ayoru
ayoru --version
```

### Install from source with Cargo

```bash
git clone https://github.com/BrandonLee28/ayoru.git
cd ayoru
cargo install --path .
```

Cargo installs the binary to `~/.cargo/bin/ayoru`.

If `ayoru` is not found in a new terminal, add Cargo's bin directory to your `PATH`.

For `zsh` on macOS:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zprofile
source ~/.zprofile
```

## Quick Start

### Watch from the CLI

```bash
ayoru "frieren"
```

This flow:

1. searches for the title
2. lets you pick a result
3. lets you pick an episode
4. resolves streams
5. opens playback in your local player

### Open the TUI

```bash
ayoru
```

The TUI includes:

- search
- favorites
- recently watched
- history
- keyboard-first navigation

`ayoru tui` also works if you prefer the explicit subcommand.

## TUI Controls

- type to search when search is focused
- `/` focus search
- `Tab` move between shell panels
- `h` / `l` move panel focus when search is not focused
- `j` / `k` or arrow keys move inside the active panel
- `Enter` confirm or select in the active flow
- `f` toggle favorite when search is not focused
- `Esc` back out of detail or playback states
- `q` quit

## Troubleshooting

### `ayoru: command not found`

The install likely succeeded, but your shell has not picked up the bin directory yet.

If you used the one-command installer, open a new terminal or reload your shell profile:

```bash
source ~/.zprofile
```

Then check again:

```bash
command -v ayoru
```

### The installer falls back to building from source

That is expected when a matching release artifact is unavailable for your platform. In that case the installer uses `cargo build --release`, so Rust and Cargo must be installed.

### Playback does not start

Make sure at least one supported player is installed:

- `mpv`
- `iina`
- `vlc`

## Local Data

The TUI stores local state at:

- `$XDG_STATE_HOME/ayoru/library.json`, or
- `~/.local/state/ayoru/library.json`

This file stores favorites, recently watched, and history in JSON.

## Current Scope

Ayoru currently focuses on:

- terminal-first playback
- one provider integration
- local player launch
- local-only saved state

It does not currently include:

- accounts
- sync
- recommendations
- collections
- a native desktop app

## Development

Useful commands:

```bash
cargo run -- --help
cargo test -q
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Repo Layout

```text
src/
  app.rs          app orchestration and runtime traits
  args.rs         command parsing
  cli/            prompt-style picker flow
  core/           models, playback policy, stream ranking
  player/         player detection and launch
  provider/       provider integrations
  tui/            TUI shell, renderer, runtime, persistence
tests/            integration and behavior tests
docs/             design and implementation docs
```

## Release Asset Naming

The installer expects release artifacts to use these names:

- `ayoru-darwin-aarch64.tar.gz`
- `ayoru-darwin-x86_64.tar.gz`
- `ayoru-linux-aarch64.tar.gz`
- `ayoru-linux-x86_64.tar.gz`

## Maintainer Release Flow

Push a version tag to publish release assets:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The GitHub Actions workflow builds the four installer tarballs and attaches them to the matching GitHub Release.

## Notes

- The CLI and TUI share the same provider and playback stack.
- Favorites, history, and recently watched are local-only by design.
- Design and implementation notes live under `docs/plans/`.
