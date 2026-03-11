# Ayoru Dashboard Shell Design

**Date:** 2026-03-11

## Goal

Evolve the current TUI from a prompt-like flow into a persistent media dashboard shell that feels more modern and workspace-oriented, while adding local quality-of-life features such as history, favorites, and recently watched.

## Product Direction

Ayoru should feel like a quiet premium media tool rather than a terminal picker. The new shell should take inspiration from structured, stateful interfaces like the Codex UI reference, but translate that structure into a media workflow:

- the center is what the user is doing now
- the side rail is what the user might want next
- the top band communicates state without becoming decorative chrome

The shell should balance two user intents:

1. start a new search
2. resume or revisit something already watched

## Core UX Shape

The TUI becomes a persistent dashboard rather than a sequence of isolated views.

### Primary Regions

- `Header band`
  - Ayoru wordmark
  - subtle mode/status line
  - focused search input
- `Main content area`
  - active search results
  - detail or episode context
  - playback transition states
- `Context rail`
  - recently watched
  - favorites
  - history

### Interaction Model

- typing enters the search field when search is focused
- arrow keys or `j/k` move inside the active panel
- `Tab` or `h/l` moves focus across major panels
- `Enter` acts on the selected item in the active panel
- `/` returns focus to search
- `Esc` backs out of detail/playback states
- `f` toggles favorite on the selected title or active item context

The active panel must always be visually clear.

## Visual Direction

The shell should feel more modern and product-like than the current TUI.

### Desired Traits

- persistent structure
- layered panels
- softer boundaries
- low-noise status surfaces
- calmer, more deliberate spacing

### Avoid

- terminal-box overload
- list-picker look and feel
- loud accent colors
- gimmicky “glass” effects in the TUI

The TUI should imply depth and polish through layout, hierarchy, and restrained styling. The future macOS app can carry the fuller glass/material expression later.

## Data Model

Store local data on disk in a simple exportable structure, likely JSON in v1.

### Records

`history`
- title id
- title name
- episode number
- watched at timestamp

`recently_watched`
- title id
- title name
- episode number
- last opened timestamp

`favorites`
- title id
- title name
- added at timestamp

### Behavioral Rules

- launching or watching an episode updates recents/history
- favorites are instant and idempotent
- startup loads saved data into the shell
- local data remains easy to export later

## V1 Scope

Ship only what makes the shell feel real and useful:

- dashboard shell layout
- search panel
- result/detail flow
- recently watched
- favorites
- history
- local persistence
- favorite toggle
- resume/open from recent items

## Explicit Non-Goals

- cloud sync
- accounts
- recommendations
- collections
- profiles
- large settings UI

## Architecture Direction

The current TUI state machine should be expanded into a shell-oriented state model rather than replaced with ad hoc conditionals.

The implementation should likely introduce:

- panel focus state
- persisted media library state
- storage service layer
- richer shell layout renderer

The current CLI path remains untouched.

## Testing Direction

Implementation must follow strict TDD.

Primary test areas:

- focus and panel navigation
- shell reducer/controller transitions
- persistence reads/writes/updates
- recent/history update behavior after playback
- favorite toggle behavior
- startup hydration from saved data

## Recommendation

Build the dashboard shell and persistence layer together, with the shell as the user-facing center of gravity and the saved media state as the product layer that makes it useful.
