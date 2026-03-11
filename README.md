# Ayoru

`ayoru` is a quieter way to watch anime.

Playback-first anime CLI MVP.

## Usage

```bash
cargo run -- "frieren"
cargo run -- tui
```

## Implemented MVP behaviors

- `ayoru <query>` command surface with `clap`
- `ayoru tui` standalone search-first TUI entrypoint
- Layered modules (`cli`, `core`, `provider`, `player`, `app`)
- Stream ranking policy: reliability (`wixmp`, `youtube`, `sharepoint`, `hianime`) -> sub preference -> resolution
- Playback timeout/fallback engine: 6-second timeout, one attempt per provider
- Player detection order: `mpv -> iina -> vlc`
- Clear typed terminal errors for no results/player/streams/playback failure
- Test suite covering ranking, fallback, provider parsing, picker input model, player detection, and failure flows

## Current limitation

Live AllAnime HTTP fetch/stream resolution is scaffolded but not yet wired in `AllAnimeProvider`; running with a real query currently exits with a provider wiring error.

The TUI reuses the same provider/playback stack, so it currently shares that limitation.

## Test and lint

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```
