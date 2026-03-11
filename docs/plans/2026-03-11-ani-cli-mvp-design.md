# ani CLI MVP Design

**Date:** 2026-03-11
**Status:** Approved
**Scope:** Playback-first MVP for `ani <query>`

## 1. Goals and Scope

### Goals
- Fast query-to-playback flow.
- Manual title and episode selection.
- Automatic stream resolution and fallback.
- Minimal command surface (`ani <query>`, `--help`, `--version`).

### Explicitly Out of Scope
- Downloads, history, resume, bookmarks.
- Config files or env-driven behavior.
- Advanced flags (`--dub`, `--source`, `--res`, `--player`, `--json`).
- Full-screen TUI and scripting-focused modes.

## 2. Chosen Approach

Selected approach: **Layered Rust MVP**.

Rationale:
- Keeps provider/business logic out of CLI UI code.
- Enables clean testing (ranking, fallback, timeout, parsing).
- Leaves a reusable core for future TUI/macOS frontends.

## 3. Architecture

### Module Layout
- `src/main.rs`: argument parsing, top-level orchestration, exit codes.
- `src/cli/`: minimal interactive picker for titles and episodes.
- `src/core/`:
  - `search.rs`: title search use case.
  - `episodes.rs`: episode fetch and ascending ordering.
  - `stream_ranker.rs`: candidate scoring/ranking.
  - `playback.rs`: player launch, playback-start timeout, fallback loop.
- `src/provider/allanime.rs`: AllAnime API integration + source extraction.
- `src/player/detect.rs`: player discovery and launch command construction.
- `src/errors.rs`: typed errors and user-facing terminal messages.
- `tests/`: unit and fixture-backed integration tests.

### Provider Strategy (copy ani-cli behavior)
- Source of truth: AllAnime/AllManga endpoints.
- Provider order for fallback attempts:
  1. `wixmp`
  2. `youtube`
  3. `sharepoint`
  4. `hianime`
- Fallback policy: **one attempt per provider** (single pass).

### Player Strategy
- Auto-detect and prefer:
  1. `mpv`
  2. `iina` (macOS)
  3. `vlc`
- If none found: print install guidance and exit.

## 4. Runtime Flow

1. Parse args; require query from CLI args.
2. Search titles using provider with `sub` preference by default.
3. Show title picker (`↑/↓`, `j/k`, `Enter`, `Esc`, `q`).
4. Fetch episodes for title; normalize and sort ascending.
5. Show episode picker with same controls.
6. Resolve stream candidates from provider payload.
7. Rank candidates by:
   1. reliability
   2. subtitle/language quality (prefer sub)
   3. resolution/bitrate
8. Detect player (`mpv -> iina -> vlc`).
9. Launch playback for top ranked candidate.
10. Treat attempt as failure unless playback starts within 6 seconds.
11. Retry next ranked provider candidate automatically.
12. Exit with clear terminal error after providers exhausted.

## 5. Error and Exit Behavior

- No query: show usage/help and exit non-zero.
- No results: print clear no-results message and exit non-zero.
- Provider/search/episode failures: clear failure message and exit non-zero.
- No supported player: install guidance and exit non-zero.
- No playable streams: clear failure message and exit non-zero.
- Playback timeout/failure: fallback automatically; final error after one-pass exhaustion.

## 6. Testing Strategy

### Unit Tests
- Ranking order honors reliability > language/sub > quality.
- Fallback loop attempts each provider at most once.
- Playback timeout marks candidate failed at 6 seconds.
- Episode ordering is strictly ascending.
- Player detection chooses `mpv`, then `iina`, then `vlc`.

### Fixture-backed Provider Tests
- Parse search results from recorded provider responses.
- Parse episode lists and stream candidates from fixtures.
- Validate provider mapping into ranked candidate set.

## 7. Acceptance Criteria

- `ani <query>` reaches interactive title picker quickly.
- User can select title and episode with minimal controls.
- Stream and player are selected automatically.
- Playback starts without manual source/resolution selection.
- If first provider fails, next provider is attempted automatically.
- User sees clear errors on terminal failure states.
