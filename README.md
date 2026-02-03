# letsgetrusty-bootcamp-jira-clone

A small, terminal-based Jira-like tracker built in Rust as a learning project. It lets you manage Epics and Stories stored in a simple JSON file and navigate between pages in a text UI.

## Overview
- Text UI (TUI) app that runs in the terminal
- Manage Epics and Stories with statuses (Open, In Progress, Resolved, Closed)
- JSON file persistence (no external DB)
- Clear separation between data models, storage layer, navigation, and pages
- Unit tests for core logic (DB layer and pages)

## Project Structure
```
├─ Cargo.toml
├─ data/
│  └─ db.json                 # JSON storage used by the app
├─ src/
│  ├─ main.rs                 # App entry point
│  ├─ models.rs               # Core domain models (Epic, Story, Status, DBState, Action)
│  ├─ db.rs                   # Storage layer (JSON-backed) + tests & mock DB
│  ├─ navigator.rs            # Navigation stack and action handling
│  ├─ io_utils.rs             # Input helper for reading user input
│  └─ ui/
│     ├─ mod.rs               # Re-exports for pages and prompts
│     ├─ prompts.rs           # Interactive prompts (create/delete/update)
│     └─ pages/
│        ├─ mod.rs            # Page trait and concrete pages: Home, EpicDetail, StoryDetail
│        └─ page_helpers.rs   # Small helpers for table-like formatting
└─ target/                    # Build artifacts
```

### Key Modules
- `models.rs`
  - `Epic`, `Story`, `Status` and `DBState` (the persisted state)
  - `Action` enum lists all navigation and mutation intents
- `db.rs`
  - `JiraDatabase` facade with JSON-backed implementation (`JSONFileDatabase`)
  - CRUD operations: create/delete epics and stories, update statuses
  - `#[cfg(test)]` mock DB and unit tests for database behavior
- `navigator.rs`
  - Manages a stack of pages and applies `Action`s to mutate state or navigate
  - Integrates with `Prompts` to obtain user-confirmation or input for mutations
- `ui/pages/mod.rs`
  - `Page` trait with `draw_page` and `handle_input`
  - Pages:
    - `HomePage`: lists epics
    - `EpicDetail`: shows one epic and its stories
    - `StoryDetail`: shows one story
- `ui/prompts.rs`
  - Encapsulates interactive prompts: create epic/story, delete confirmations, update status selection
- `ui/pages/page_helpers.rs`
  - `get_column_string` helper to format fixed-width columns in the TUI

## Data Storage
- The app reads and writes a pretty-printed JSON file at `data/db.json`.
- Expected minimal schema (example):
  ```json
  {
    "last_item_id": 0,
    "epics": {},
    "stories": {}
  }
  ```
- If the file is missing or contains invalid JSON, the app will error at startup. Create it before running.

## Build and Run
Prerequisites: Rust (edition 2021) and Cargo installed.

- Build:
  ```bash
  cargo build
  ```
- Run:
  ```bash
  cargo run
  ```

The app expects `data/db.json` to exist (see Data Storage above).

## Usage (Key Bindings)
When the app draws a page, type a command and press Enter.

- Home page
  - Display: list of epics
  - Commands: `q` quit | `c` create epic | `:id:` navigate to epic (type the numeric epic id)
- Epic detail page
  - Display: selected epic header and its stories
  - Commands: `p` previous | `u` update epic | `d` delete epic | `c` create story | `:id:` navigate to story by id
- Story detail page
  - Display: selected story
  - Commands: `p` previous | `u` update story | `d` delete story

Prompts will ask for names/descriptions and confirm deletes. Status updates map numbers to statuses: `1` OPEN, `2` IN-PROGRESS, `3` RESOLVED, `4` CLOSED.

## Testing
Run all tests (DB layer and page behavior):
```bash
cargo test
```

## Dependencies
- `anyhow`: simple error handling
- `serde`, `serde_json`: JSON serialization/deserialization
- `ellipse`: string truncation with ellipsis for column formatting
- `itertools`: iteration helpers (sorting/formatting)
- `clearscreen`: clear terminal between page draws
- `tempfile` (dev): used in tests

## Notes
- This repo is part of a bootcamp exercise; functionality is intentionally scoped and focused on fundamentals.
- The UI is intentionally simple for clarity and learning; it uses standard input/output and avoids terminal control libraries.
