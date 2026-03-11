# ani (Rust MVP)

Playback-first anime CLI MVP.

## Usage

```bash
cargo run -- "frieren"
```

## Implemented MVP behaviors

- `ani <query>` command surface with `clap`
- Layered modules (`cli`, `core`, `provider`, `player`, `app`)
- Stream ranking policy: reliability (`wixmp`, `youtube`, `sharepoint`, `hianime`) -> sub preference -> resolution
- Playback timeout/fallback engine: 6-second timeout, one attempt per provider
- Player detection order: `mpv -> iina -> vlc`
- Clear typed terminal errors for no results/player/streams/playback failure
- Test suite covering ranking, fallback, provider parsing, picker input model, player detection, and failure flows

## Current limitation

Live AllAnime HTTP fetch/stream resolution is scaffolded but not yet wired in `AllAnimeProvider`; running with a real query currently exits with a provider wiring error.

## Test and lint

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```
