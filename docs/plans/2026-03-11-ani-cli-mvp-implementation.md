# ani CLI MVP Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.
> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:tdd-workflow for every implementation task (red-green-refactor).

**Goal:** Build a Rust CLI MVP that runs `ani <query>`, lets users pick title + episode, auto-resolves and plays streams, and falls back across ani-cli provider order.

**Architecture:** Implement a layered Rust app with separate `cli`, `core`, `provider`, and `player` modules. Keep provider/business logic reusable and testable outside UI. Use fixture-backed provider parsing and deterministic fallback/playback policy.

**Tech Stack:** Rust (stable), `clap`, `tokio`, `reqwest`, `serde`, `ratatui` + `crossterm` (minimal picker), `which`, `anyhow`/`thiserror`, `wiremock` (or fixture-only parser tests)

---

### Task 1: Bootstrap Rust CLI crate and command surface

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/args.rs`
- Test: `tests/cli_args.rs`

**Step 1: Write the failing test**

```rust
// tests/cli_args.rs
#[test]
fn rejects_missing_query() {
    let err = ani::args::parse_from(["ani"]);
    assert!(err.is_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test rejects_missing_query -q`
Expected: FAIL (crate/module missing)

**Step 3: Write minimal implementation**

```rust
// src/args.rs
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub query: Vec<String>,
}

pub fn parse_from<I, T>(itr: I) -> Result<Args, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = Args::try_parse_from(itr)?;
    if args.query.is_empty() {
        return Err(clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, "query is required"));
    }
    Ok(args)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test rejects_missing_query -q`
Expected: PASS

**Step 5: Commit**

```bash
git add Cargo.toml src/main.rs src/args.rs tests/cli_args.rs
git commit -m "feat: bootstrap rust cli args parsing"
```

### Task 2: Add domain models and stream ranking policy

**Files:**
- Create: `src/core/models.rs`
- Create: `src/core/stream_ranker.rs`
- Test: `tests/stream_ranker.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn ranks_by_provider_reliability_first() {
    // hianime has higher resolution but wixmp must win by reliability policy
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test ranks_by_provider_reliability_first -q`
Expected: FAIL (ranker missing)

**Step 3: Write minimal implementation**

```rust
pub fn provider_rank(name: &str) -> u8 {
    match name {
        "wixmp" => 0,
        "youtube" => 1,
        "sharepoint" => 2,
        "hianime" => 3,
        _ => 10,
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test --test stream_ranker -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/core/models.rs src/core/stream_ranker.rs tests/stream_ranker.rs
git commit -m "feat: add stream ranking policy for mvp"
```

### Task 3: Implement AllAnime provider client (search + episodes + stream candidates)

**Files:**
- Create: `src/provider/mod.rs`
- Create: `src/provider/allanime.rs`
- Create: `tests/fixtures/search.json`
- Create: `tests/fixtures/episodes.json`
- Create: `tests/fixtures/streams.json`
- Test: `tests/provider_allanime.rs`

**Step 1: Write the failing tests**

```rust
#[test]
fn parses_search_results_from_fixture() {}

#[test]
fn parses_episode_list_integers() {}

#[test]
fn parses_provider_candidates() {}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --test provider_allanime -q`
Expected: FAIL (provider module missing)

**Step 3: Write minimal implementation**

```rust
pub const ALLANIME_REFERER: &str = "https://allmanga.to";
pub const ALLANIME_BASE: &str = "allanime.day";
pub const ALLANIME_API: &str = "https://api.allanime.day";
```

Implement parsing for fixture payloads into `Title`, `Episode`, `StreamCandidate`.

**Step 4: Run tests to verify they pass**

Run: `cargo test --test provider_allanime -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/provider/mod.rs src/provider/allanime.rs tests/provider_allanime.rs tests/fixtures/*.json
git commit -m "feat: add allanime provider parsing"
```

### Task 4: Build minimal interactive pickers for title and episode

**Files:**
- Create: `src/cli/picker.rs`
- Create: `src/cli/mod.rs`
- Test: `tests/picker_navigation.rs`

**Step 1: Write the failing tests**

```rust
#[test]
fn j_and_down_move_selection_forward() {}

#[test]
fn q_and_esc_cancel_picker() {}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --test picker_navigation -q`
Expected: FAIL

**Step 3: Write minimal implementation**

Implement picker state machine that supports `Up/Down`, `j/k`, `Enter`, `Esc`, `q` and returns `Selected(index)` or `Cancelled`.

**Step 4: Run tests to verify they pass**

Run: `cargo test --test picker_navigation -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/cli/mod.rs src/cli/picker.rs tests/picker_navigation.rs
git commit -m "feat: add minimal interactive picker state"
```

### Task 5: Add player detection and launch adapter

**Files:**
- Create: `src/player/detect.rs`
- Create: `src/player/launch.rs`
- Create: `src/player/mod.rs`
- Test: `tests/player_detection.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn chooses_mpv_then_iina_then_vlc() {}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test player_detection -q`
Expected: FAIL

**Step 3: Write minimal implementation**

Implement detection order with `which`:
1. `mpv`
2. `iina`
3. `vlc`

Return install guidance error if none found.

**Step 4: Run test to verify it passes**

Run: `cargo test --test player_detection -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/player/mod.rs src/player/detect.rs src/player/launch.rs tests/player_detection.rs
git commit -m "feat: add player detection and launch abstraction"
```

### Task 6: Implement playback watchdog and provider fallback loop

**Files:**
- Create: `src/core/playback.rs`
- Modify: `src/core/stream_ranker.rs`
- Test: `tests/playback_fallback.rs`

**Step 1: Write the failing tests**

```rust
#[tokio::test]
async fn fails_candidate_if_playback_not_started_within_6s() {}

#[tokio::test]
async fn tries_each_provider_once_then_fails() {}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --test playback_fallback -q`
Expected: FAIL

**Step 3: Write minimal implementation**

Implement:
- `tokio::time::timeout(Duration::from_secs(6), start_playback(...))`
- single-pass fallback over ranked unique providers
- terminal failure error after exhaustion

**Step 4: Run tests to verify they pass**

Run: `cargo test --test playback_fallback -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/core/playback.rs src/core/stream_ranker.rs tests/playback_fallback.rs
git commit -m "feat: add playback timeout and fallback policy"
```

### Task 7: Wire end-to-end orchestration in `main` and validate failures

**Files:**
- Modify: `src/main.rs`
- Create: `src/errors.rs`
- Test: `tests/mvp_flow.rs`

**Step 1: Write the failing tests**

```rust
#[test]
fn exits_cleanly_on_no_results() {}

#[test]
fn exits_with_install_guidance_when_no_player_found() {}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --test mvp_flow -q`
Expected: FAIL

**Step 3: Write minimal implementation**

Wire full flow:
- parse query
- search
- title picker
- episode picker (ascending)
- resolve/rank
- detect player
- playback with fallback
- clear terminal errors on failure states

**Step 4: Run tests to verify they pass**

Run: `cargo test --test mvp_flow -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/main.rs src/errors.rs tests/mvp_flow.rs
git commit -m "feat: wire mvp search-to-play flow"
```

### Task 8: Verification gate before completion

**Files:**
- Modify: `README.md` (if created)

**Step 1: Run full test suite**

Run: `cargo test`
Expected: all tests PASS

**Step 2: Run lint and formatting**

Run: `cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings`
Expected: no formatting diffs, no clippy warnings

**Step 3: Manual smoke test**

Run: `cargo run -- "frieren"`
Expected: title picker -> episode picker -> playback attempt/fallback behavior

**Step 4: Document MVP usage**

Add concise usage and supported players/providers.

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: add mvp usage and behavior notes"
```
