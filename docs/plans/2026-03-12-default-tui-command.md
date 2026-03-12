# Default TUI Command Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Route bare `ayoru` to the TUI while keeping `ayoru "<query>"` as the direct CLI mode.

**Architecture:** Update argument parsing so zero positional arguments become `Command::Tui`. Keep `main` dispatch unchanged except for consuming the new parse result. Update command usage text and tests to match the new default behavior.

**Tech Stack:** Rust, clap, integration-style argument tests

---

### Task 1: Update command parsing

**Files:**
- Modify: `src/args.rs`
- Test: `tests/cli_args.rs`

**Step 1: Write the failing test**

Add a test asserting `parse_from(["ayoru"])` returns `Command::Tui`.

**Step 2: Run test to verify it fails**

Run: `cargo test --test cli_args`
Expected: FAIL because bare `ayoru` still returns a missing query error.

**Step 3: Write minimal implementation**

Change zero-argument parsing to return `Command::Tui` and update usage/help text accordingly.

**Step 4: Run test to verify it passes**

Run: `cargo test --test cli_args`
Expected: PASS

**Step 5: Review docs**

Confirm README command examples remain consistent with the new behavior. Update only if necessary.
