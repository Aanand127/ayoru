# Ayoru Dashboard Shell Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a persistent Ayoru dashboard shell with local history, favorites, and recently watched while keeping the existing CLI flow untouched.

**Architecture:** Expand the current `src/tui/` state/controller/runtime into a panel-based shell and add a small persistence layer under `src/tui/` or `src/core/` for local watch data. Keep the old `ani <query>` CLI path unchanged and drive all new behavior through the `ani tui` path.

**Tech Stack:** Rust (stable), `ratatui`, `crossterm`, `tokio`, existing provider/player/core modules, JSON file persistence via `serde`/`serde_json`

---

### Task 1: Add a persisted media-state model for history, favorites, and recently watched

**Files:**
- Create: `src/tui/library.rs`
- Modify: `src/tui/mod.rs`
- Test: `tests/tui_library.rs`

**Step 1: Write the failing test**

Add tests for the data model only:

```rust
#[test]
fn toggling_favorite_is_idempotent() {
    let mut library = LibraryState::default();
    let title = saved_title("show-1", "Frieren");

    library.toggle_favorite(title.clone());
    library.toggle_favorite(title.clone());

    assert!(library.favorites.is_empty());
}
```

```rust
#[test]
fn recording_watch_updates_history_and_recently_watched() {
    let mut library = LibraryState::default();
    library.record_watch(saved_watch("show-1", "Frieren", 3));

    assert_eq!(library.history.len(), 1);
    assert_eq!(library.recently_watched.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_library -q`
Expected: FAIL because `src/tui/library.rs` does not exist.

**Step 3: Write minimal implementation**

- Create `LibraryState`, `SavedTitle`, and `SavedWatch`
- Add `toggle_favorite(...)`
- Add `record_watch(...)`
- Export the module from `src/tui/mod.rs`

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_library -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/tui/library.rs tests/tui_library.rs
git commit -m "feat: add tui media library state"
```

### Task 2: Add JSON persistence for the media library

**Files:**
- Modify: `src/tui/library.rs`
- Create: `src/tui/storage.rs`
- Modify: `src/tui/mod.rs`
- Test: `tests/tui_storage.rs`

**Step 1: Write the failing test**

Add persistence tests using a temp directory:

```rust
#[test]
fn saves_and_loads_library_state_as_json() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("library.json");
    let storage = LibraryStorage::new(path.clone());

    let mut library = LibraryState::default();
    library.record_watch(saved_watch("show-1", "Frieren", 1));

    storage.save(&library).unwrap();
    let loaded = storage.load().unwrap();

    assert_eq!(loaded.history.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_storage -q`
Expected: FAIL because storage and serialization are missing.

**Step 3: Write minimal implementation**

- Add `serde` derives to persisted structs
- Add `LibraryStorage` with `load()` and `save()`
- Default missing file to an empty `LibraryState`

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_storage -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/library.rs src/tui/storage.rs src/tui/mod.rs tests/tui_storage.rs Cargo.toml Cargo.lock
git commit -m "feat: persist tui library state"
```

### Task 3: Expand the TUI state machine into a shell with panel focus

**Files:**
- Modify: `src/tui/state.rs`
- Modify: `src/tui/action.rs`
- Test: `tests/tui_shell_state.rs`

**Step 1: Write the failing test**

Add panel-focus reducer tests:

```rust
#[test]
fn tab_moves_focus_from_search_to_context_rail() {
    let mut state = TuiState::default();

    state.apply(Action::FocusNextPanel);

    assert_eq!(state.focused_panel, Panel::ContextRail);
}
```

```rust
#[test]
fn favorite_action_marks_selected_result() {
    let mut state = search_state_with_results();

    let effect = state.apply(Action::ToggleFavorite);

    assert_eq!(effect, Some(Effect::ToggleFavoriteForSelectedTitle));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_shell_state -q`
Expected: FAIL because panel state and favorite actions do not exist.

**Step 3: Write minimal implementation**

- Add `Panel` enum
- Track focused panel in `TuiState`
- Add shell actions:
  - `FocusNextPanel`
  - `FocusPrevPanel`
  - `ToggleFavorite`
  - `OpenRecentItem`
- Emit effects for favorite/recent interactions

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_shell_state -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/state.rs src/tui/action.rs tests/tui_shell_state.rs
git commit -m "feat: add shell panel focus state"
```

### Task 4: Wire library persistence into the controller lifecycle

**Files:**
- Modify: `src/tui/controller.rs`
- Modify: `src/tui/library.rs`
- Modify: `src/tui/storage.rs`
- Test: `tests/tui_shell_controller.rs`

**Step 1: Write the failing test**

Add controller tests for startup hydration and favorite toggling:

```rust
#[tokio::test]
async fn controller_loads_saved_library_state_on_startup() {
    let storage = seeded_storage_with_recent("Frieren", 2);
    let app = TuiController::with_storage(provider(), player(), storage).await.unwrap();

    assert_eq!(app.library().recently_watched.len(), 1);
}
```

```rust
#[tokio::test]
async fn toggling_favorite_updates_library_and_persists() {
    let mut app = seeded_search_controller();

    app.dispatch(Action::ToggleFavorite).await.unwrap();

    assert_eq!(app.library().favorites.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_shell_controller -q`
Expected: FAIL because the controller does not own persisted library state.

**Step 3: Write minimal implementation**

- Inject storage into the controller
- Load persisted library on startup
- Save after favorite and watch updates
- Expose read-only accessors for tests

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_shell_controller -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/controller.rs src/tui/library.rs src/tui/storage.rs tests/tui_shell_controller.rs
git commit -m "feat: wire shell library persistence"
```

### Task 5: Update playback flow to record history and recently watched

**Files:**
- Modify: `src/tui/controller.rs`
- Modify: `src/tui/library.rs`
- Test: `tests/tui_playback_flow.rs`

**Step 1: Write the failing test**

Add a playback success/failure persistence test:

```rust
#[tokio::test]
async fn successful_playback_records_recent_and_history() {
    let mut app = seeded_playable_controller();

    app.dispatch(Action::PlaySelectedEpisode).await.unwrap();

    assert_eq!(app.library().history.len(), 1);
    assert_eq!(app.library().recently_watched.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_playback_flow -q`
Expected: FAIL because playback does not yet record library updates.

**Step 3: Write minimal implementation**

- On successful playback, record watch state
- Keep failure behavior recoverable
- Persist the update through storage

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_playback_flow -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/controller.rs src/tui/library.rs tests/tui_playback_flow.rs
git commit -m "feat: record recents and history from playback"
```

### Task 6: Redesign the renderer into a shell layout with a context rail

**Files:**
- Modify: `src/tui/ui.rs`
- Test: `tests/tui_shell_render.rs`

**Step 1: Write the failing test**

Add render tests for shell presence:

```rust
#[test]
fn shell_render_shows_search_header_and_context_sections() {
    let buffer = render_to_buffer(&shell_fixture_state(), 120, 32);

    assert!(buffer_contains(&buffer, "AYORU"));
    assert!(buffer_contains(&buffer, "Recently watched"));
    assert!(buffer_contains(&buffer, "Favorites"));
    assert!(buffer_contains(&buffer, "History"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_shell_render -q`
Expected: FAIL because the renderer only supports the current simple three-block layout.

**Step 3: Write minimal implementation**

- Add a true shell layout:
  - header band
  - main content panel
  - side context rail
- Make active panel focus visually obvious
- Keep the neutral premium Ayoru palette

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_shell_render -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/ui.rs tests/tui_shell_render.rs
git commit -m "feat: render ayoru dashboard shell"
```

### Task 7: Add keyboard support for panel navigation and favorites

**Files:**
- Modify: `src/tui/runtime.rs`
- Modify: `src/tui/action.rs`
- Test: `tests/tui_input.rs`

**Step 1: Write the failing test**

Add tests for shell-specific input:

```rust
#[test]
fn tab_moves_between_shell_panels() {
    let state = shell_fixture_state();
    assert_eq!(map_key_code_for_state(&state, KeyCode::Tab), Some(InputCommand::Action(Action::FocusNextPanel)));
}
```

```rust
#[test]
fn f_toggles_favorite_in_shell_context() {
    let state = shell_fixture_state();
    assert_eq!(map_key_code_for_state(&state, KeyCode::Char('f')), Some(InputCommand::Action(Action::ToggleFavorite)));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_input -q`
Expected: FAIL because shell navigation and favorite bindings are missing.

**Step 3: Write minimal implementation**

- Add `Tab`, `h`, `l`, and `f` bindings
- Keep search text entry behavior intact when search is focused
- Route actions through the shell controller

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_input -q`
Expected: PASS

**Step 5: Commit**

```bash
git add src/tui/runtime.rs src/tui/action.rs tests/tui_input.rs
git commit -m "feat: add shell keyboard controls"
```

### Task 8: Verify end-to-end shell startup with saved data

**Files:**
- Modify: `tests/tui_runtime.rs`
- Modify: `tests/tui_shell_controller.rs`

**Step 1: Write the failing test**

Add a startup test that proves saved favorites/history appear in the shell-ready controller state.

**Step 2: Run test to verify it fails**

Run: `cargo test --test tui_runtime --test tui_shell_controller -q`
Expected: FAIL until startup hydration and shell wiring are complete.

**Step 3: Write minimal implementation**

- Finish any missing startup plumbing
- Keep runtime teardown behavior intact

**Step 4: Run test to verify it passes**

Run: `cargo test --test tui_runtime --test tui_shell_controller -q`
Expected: PASS

**Step 5: Commit**

```bash
git add tests/tui_runtime.rs tests/tui_shell_controller.rs src/tui/controller.rs src/tui/runtime.rs
git commit -m "test: verify shell startup hydration"
```

### Task 9: Final verification and user-facing docs

**Files:**
- Modify: `README.md`
- Test: `tests/cli_args.rs`

**Step 1: Write the failing test**

Add or update a test that confirms the `ani tui` command shape still works with the shell changes.

**Step 2: Run test to verify it fails**

Run: `cargo test --test cli_args -q`
Expected: FAIL if command wiring drifted during the shell changes.

**Step 3: Write minimal implementation**

- Update `README.md` to mention:
  - dashboard shell
  - history/favorites/recently watched
  - local saved state
- Keep CLI docs accurate and minimal

**Step 4: Run test to verify it passes**

Run:
- `cargo test -q`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`

Expected: all PASS

**Step 5: Commit**

```bash
git add README.md tests/cli_args.rs
git commit -m "docs: add ayoru shell usage notes"
```
