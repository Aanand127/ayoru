# ani TUI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a standalone `ani tui` terminal UI with a search-first, keyboard-driven flow that reuses existing provider/playback logic without changing the current `ani <query>` CLI behavior.

**Architecture:** Add a separate `src/tui/` frontend with its own state machine, renderer, and event loop. Wire it behind a new `ani tui` subcommand while keeping the existing CLI path intact. Reuse provider, ranking, and player-launch code directly from the TUI controller rather than forcing the flow through `PickerRuntime`.

**Tech Stack:** Rust (stable), `clap`, `tokio`, `crossterm`, `ratatui`, existing `ani` app/core/provider/player modules

---

### Task 1: Add the `ani tui` command surface without changing the current CLI flow

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/args.rs`
- Modify: `src/main.rs`
- Modify: `src/lib.rs`
- Test: `tests/cli_args.rs`

**Step 1: Write the failing test**

Add a test in `tests/cli_args.rs` proving:

```rust
#[test]
fn parses_tui_subcommand_without_query() {
    let args = ani::args::parse_from(["ani", "tui"]).unwrap();
    assert!(matches!(args.command, ani::args::Command::Tui));
}
```

Add a second test proving the existing query mode still parses:

```rust
#[test]
fn parses_query_mode_unchanged() {
    let args = ani::args::parse_from(["ani", "frieren"]).unwrap();
    assert!(matches!(args.command, ani::args::Command::Play { .. }));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test cli_args -q`
Expected: FAIL because `Args` does not yet expose a `command` field or `Tui` variant.

**Step 3: Write minimal implementation**

- Add `ratatui = "0.29"` to `Cargo.toml`.
- Replace the current `Args { query: Vec<String> }` shape with a subcommand-based model:

```rust
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Tui,
    Play { query: Vec<String> },
}
```

- Preserve the current `ani <query>` UX by using clap configuration that treats bare trailing values as the default play path.
- Update `src/main.rs` to branch:
  - `ani tui` -> call a new `ani::tui::run().await`
  - bare query mode -> execute the current CLI flow unchanged
- Export `pub mod tui;` from `src/lib.rs`

**Step 4: Run test to verify it passes**

Run: `cargo test --test cli_args -q`
Expected: PASS

**Step 5: Commit**

```bash
git add Cargo.toml src/args.rs src/main.rs src/lib.rs tests/cli_args.rs
git commit -m "feat: add ani tui command surface"
```

### Task 2: Create a minimal TUI bootstrap that enters and exits fullscreen safely

**Files:**
- Create: `src/tui/mod.rs`
- Create: `src/tui/runtime.rs`
- Modify: `src/main.rs`
- Test: `tests/tui_runtime.rs`

**Step 1: Write the failing test**

Add a runtime-focused test around a terminal abstraction rather than real stdout:

```rust
#[tokio::test]
async fn tui_runtime_restores_terminal_on_clean_exit() {
    let mut terminal = FakeTerminal::default();
    ani::tui::runtime::run_with_terminal(&mut terminal, StubApp::quit_immediately())
        .await
        .unwrap();

    assert!(terminal.entered_alt_screen);
    assert!(terminal.left_alt_screen);
    assert!(terminal.raw_mode_disabled);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_runtime -q`
Expected: FAIL because `ani::tui::runtime` and the terminal abstraction do not exist yet.

**Step 3: Write minimal implementation**

- Add `src/tui/mod.rs` exporting `pub mod runtime;`
- Define a small terminal abstraction in `src/tui/runtime.rs` so lifecycle can be tested without drawing real frames.
- Implement:
  - raw mode enable/disable
  - alternate screen enter/leave
  - a loop hook that runs an app object until it signals quit
- Add `pub async fn run() -> Result<(), AppError>` that wires the real terminal backend.

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_runtime -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/tui/runtime.rs tests/tui_runtime.rs src/main.rs
git commit -m "feat: add tui runtime bootstrap"
```

### Task 3: Build the TUI state machine for search mode

**Files:**
- Create: `src/tui/state.rs`
- Create: `src/tui/action.rs`
- Modify: `src/tui/mod.rs`
- Test: `tests/tui_state.rs`

**Step 1: Write the failing test**

Add reducer-oriented tests such as:

```rust
#[test]
fn submit_search_sets_loading_state() {
    let mut state = TuiState::default();
    state.query = "frieren".into();

    let effect = state.apply(Action::SubmitSearch);

    assert_eq!(state.mode, Mode::Search);
    assert!(state.is_loading);
    assert_eq!(effect, Some(Effect::SearchTitles("frieren".into())));
}
```

```rust
#[test]
fn typing_updates_query_without_side_effects() {
    let mut state = TuiState::default();

    let effect = state.apply(Action::InsertChar('f'));

    assert_eq!(state.query, "f");
    assert_eq!(effect, None);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_state -q`
Expected: FAIL because the TUI state/action types do not exist yet.

**Step 3: Write minimal implementation**

- Introduce:
  - `Mode` enum
  - `TuiState` struct with query, loading flag, search results, selection indexes, and transient message fields
  - `Action` enum for keyboard and async events
  - `Effect` enum for async intents triggered by the reducer
- Implement `TuiState::apply(...)` to mutate state and optionally emit an effect.

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_state -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/tui/state.rs src/tui/action.rs tests/tui_state.rs
git commit -m "feat: add tui search state machine"
```

### Task 4: Add title search integration to the TUI controller

**Files:**
- Create: `src/tui/controller.rs`
- Modify: `src/tui/state.rs`
- Modify: `src/tui/mod.rs`
- Test: `tests/tui_search_flow.rs`

**Step 1: Write the failing test**

Add an integration-style test using a scripted provider:

```rust
#[tokio::test]
async fn search_success_populates_results_and_selects_first_item() {
    let provider = SearchProvider::with_titles(vec!["Frieren", "FMA"]);
    let mut app = TuiController::new(provider, NoopPlayerRuntime);

    app.dispatch(Action::InsertString("frie".into())).await;
    app.dispatch(Action::SubmitSearch).await;

    assert_eq!(app.state().results.len(), 2);
    assert_eq!(app.state().selected_result, 0);
    assert!(!app.state().is_loading);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_search_flow -q`
Expected: FAIL because the controller/provider integration is not wired.

**Step 3: Write minimal implementation**

- Add a TUI controller that:
  - owns `TuiState`
  - receives reducer effects
  - calls `ProviderRuntime::search(...)`
  - feeds success/failure back into state
- Keep the controller generic over provider/player dependencies so it is testable with scripted fakes.

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_search_flow -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/tui/controller.rs src/tui/state.rs tests/tui_search_flow.rs
git commit -m "feat: wire tui search flow"
```

### Task 5: Add episode loading and back navigation with state preservation

**Files:**
- Modify: `src/tui/controller.rs`
- Modify: `src/tui/state.rs`
- Test: `tests/tui_episode_flow.rs`

**Step 1: Write the failing test**

Add coverage for title selection, episode load, and back navigation:

```rust
#[tokio::test]
async fn opening_episodes_preserves_search_query_when_navigating_back() {
    let provider = EpisodeProvider::fixture();
    let mut app = TuiController::new(provider, NoopPlayerRuntime);

    app.seed_search_results(vec!["Frieren"]);
    app.dispatch(Action::OpenSelectedTitle).await;

    assert_eq!(app.state().mode, Mode::Episodes);
    assert_eq!(app.state().episodes.len(), 3);

    app.dispatch(Action::Back).await;

    assert_eq!(app.state().mode, Mode::Search);
    assert_eq!(app.state().query, "frieren");
    assert_eq!(app.state().selected_result, 0);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_episode_flow -q`
Expected: FAIL because episode loading/back behavior is incomplete.

**Step 3: Write minimal implementation**

- Extend reducer/controller flow for:
  - `OpenSelectedTitle`
  - `EpisodesLoaded`
  - `Back`
- Preserve search query, search results, and selected index when returning from episodes.

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_episode_flow -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/controller.rs src/tui/state.rs tests/tui_episode_flow.rs
git commit -m "feat: add tui episode navigation"
```

### Task 6: Add playback launch flow and recoverable failure handling

**Files:**
- Modify: `src/tui/controller.rs`
- Modify: `src/tui/state.rs`
- Test: `tests/tui_playback_flow.rs`

**Step 1: Write the failing test**

Add tests for both success and recoverable failure:

```rust
#[tokio::test]
async fn play_episode_enters_launching_mode_and_returns_to_episodes_on_failure() {
    let provider = PlaybackProvider::fixture();
    let player = FailingPlayerRuntime::default();
    let mut app = TuiController::new(provider, player);

    app.seed_episode_context();
    app.dispatch(Action::PlaySelectedEpisode).await;

    assert_eq!(app.state().mode, Mode::Episodes);
    assert_eq!(app.state().message.as_deref(), Some("Playback failed after trying all providers"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_playback_flow -q`
Expected: FAIL because playback orchestration is not yet implemented.

**Step 3: Write minimal implementation**

- Reuse existing stream fetch, `rank_streams(...)`, player detect, and launch/fallback logic.
- Keep playback failures recoverable inside the TUI by storing a message and restoring the episode screen.
- Only bubble terminal-lifecycle errors out of the top-level runtime.

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_playback_flow -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/controller.rs src/tui/state.rs tests/tui_playback_flow.rs
git commit -m "feat: add tui playback flow"
```

### Task 7: Render the search and episode screens with `ratatui`

**Files:**
- Create: `src/tui/ui.rs`
- Modify: `src/tui/runtime.rs`
- Modify: `src/tui/mod.rs`
- Test: `tests/tui_render.rs`

**Step 1: Write the failing test**

Add a focused rendering test against a `ratatui::buffer::Buffer`:

```rust
#[test]
fn render_search_screen_shows_query_results_and_key_hints() {
    let state = TuiState::search_fixture();
    let buffer = ani::tui::ui::render_to_buffer(&state, 80, 24);

    assert!(buffer_contains(&buffer, "Search"));
    assert!(buffer_contains(&buffer, "frieren"));
    assert!(buffer_contains(&buffer, "Enter"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_render -q`
Expected: FAIL because rendering helpers do not exist yet.

**Step 3: Write minimal implementation**

- Compose the top bar, main pane, and bottom status bar with `ratatui`.
- Render search results and episodes from `TuiState`.
- Keep layout simple and functional; do not add extra panels or log surfaces in v1.

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_render -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/tui/ui.rs src/tui/runtime.rs tests/tui_render.rs
git commit -m "feat: render tui screens"
```

### Task 8: Wire keyboard input into actions and finish the real event loop

**Files:**
- Modify: `src/tui/runtime.rs`
- Modify: `src/tui/action.rs`
- Modify: `src/tui/controller.rs`
- Test: `tests/tui_input.rs`

**Step 1: Write the failing test**

Add input-mapping tests around a small event translation layer:

```rust
#[test]
fn slash_and_text_input_focus_search_and_append_query() {
    assert_eq!(map_key('/'), Some(Action::FocusSearch));
    assert_eq!(map_key('f'), Some(Action::InsertChar('f')));
}
```

```rust
#[test]
fn navigation_keys_map_to_selection_actions() {
    assert_eq!(map_special(KeyCode::Down), Some(Action::MoveDown));
    assert_eq!(map_special(KeyCode::Char('k')), Some(Action::MoveUp));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_input -q`
Expected: FAIL because the input translation layer is incomplete.

**Step 3: Write minimal implementation**

- Translate `crossterm` events into TUI actions.
- Drive controller dispatch from the main event loop.
- Support:
  - typing and `/`
  - `j/k` and arrows
  - `Enter`
  - `Esc`
  - `q`

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_input -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/runtime.rs src/tui/action.rs src/tui/controller.rs tests/tui_input.rs
git commit -m "feat: wire tui keyboard controls"
```

### Task 9: Add regression coverage proving the old CLI path still works

**Files:**
- Modify: `tests/mvp_flow.rs`
- Test: `tests/mvp_flow.rs`

**Step 1: Write the failing test**

Add a CLI regression test that exercises the current non-TUI path through `run_with(...)` or argument parsing and confirms nothing changed for the existing flow.

```rust
#[tokio::test]
async fn classic_cli_path_remains_unchanged_after_tui_addition() {
    // preserve existing query-driven flow expectations
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test mvp_flow -q`
Expected: FAIL until the new argument wiring and entry handling preserve the old behavior completely.

**Step 3: Write minimal implementation**

- Adjust only the command dispatch glue needed to keep the old path intact.
- Do not rewrite app/core/cli runtime behavior unless the regression test proves it is necessary.

**Step 4: Run test to verify it passes**

Run: `cargo test --test mvp_flow -q`
Expected: PASS

**Step 5: Commit**

```bash
git add tests/mvp_flow.rs src/main.rs src/args.rs
git commit -m "test: lock cli behavior after tui addition"
```

### Task 10: Verify the full feature set and document user-facing usage

**Files:**
- Modify: `README.md`
- Test: `tests/cli_args.rs`

**Step 1: Write the failing test**

Add or update a CLI args test that documents the final `ani tui` entrypoint and rejects malformed subcommand/query combinations if needed.

**Step 2: Run test to verify it fails**

Run: `cargo test --test cli_args -q`
Expected: FAIL until the final command surface is correct.

**Step 3: Write minimal implementation**

- Update `README.md` usage examples to show both:
  - `ani "frieren"`
  - `ani tui`
- Keep docs explicit that the TUI is standalone and the old CLI still exists.

**Step 4: Run test to verify it passes**

Run:
- `cargo test --test cli_args -q`
- `cargo test -q`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`

Expected: all PASS

**Step 5: Commit**

```bash
git add README.md tests/cli_args.rs
git commit -m "docs: add tui usage and verify build"
```
