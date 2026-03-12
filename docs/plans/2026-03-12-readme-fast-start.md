# README Fast Start Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rewrite `README.md` so new users can install and run Ayoru quickly from the GitHub page.

**Architecture:** Keep the README as a single document, but reorganize it around the user journey: understand the tool, confirm requirements, install, verify, and run. Move contributor and release details lower so they do not block onboarding.

**Tech Stack:** Markdown, shell command examples, existing repository documentation

---

### Task 1: Rework README structure and copy

**Files:**
- Modify: `README.md`

**Step 1: Draft the new section order**

Use this order:
- title and one-line value proposition
- short feature summary
- requirements
- install
- quick start
- TUI controls
- troubleshooting
- local data
- development
- release notes

**Step 2: Rewrite install and quick start**

Update install copy so it:
- explains the one-command installer clearly
- shows verification commands immediately after install
- includes source install as a fallback path
- uses concrete examples for CLI and TUI

**Step 3: Add troubleshooting**

Cover:
- command not found after install
- no supported player installed
- release install fallback behavior

**Step 4: Review for readability**

Check that:
- first screen is scannable
- sections are short
- the README can be used without reading repo internals first

**Step 5: Verify**

Run: `sed -n '1,260p' README.md`
Expected: install and quick start appear before development and release details
