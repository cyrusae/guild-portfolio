# Build Plan

Atomic build steps for the issue tracker CLI. Each step ends with code that
compiles and can be manually verified. Steps build on each other — don't skip.

**Crates we'll use** (brief rationale):
- `clap` (derive feature) — parses CLI arguments and generates `--help` text
- `serde` + `serde_json` — serializes/deserializes Rust structs to/from JSON
- `chrono` — ISO 8601 timestamps for timeline events

---

## Phase 0: Rust Setup

### Step 0.1 — Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (default install is fine). Then open a new terminal or run:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version   # e.g. rustc 1.78.0
cargo --version   # e.g. cargo 1.78.0
```

### Step 0.2 — Create the project

From inside the `tracker/` directory (this repo):

```bash
cargo init --name tracker
```

This creates `Cargo.toml` and `src/main.rs`. The default `main.rs` prints
"Hello, world!" — that's fine for now.

Verify it compiles and runs:

```bash
cargo run
# Hello, world!
```

### Step 0.3 — Add dependencies

Edit `Cargo.toml` to add the crates under `[dependencies]`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
```

Verify they download and the project still compiles:

```bash
cargo build
# Downloading + compiling dependencies (first time is slow, ~30s)
```

---

## Phase 1: Data Model

No CLI yet — just the Rust structs and file I/O logic. Getting this layer right
before adding commands means we only define the data shape once.

### Step 1.1 — Define the core structs

Create `src/data.rs`. Define `TrackerFile`, `Issue`, `TimelineEvent`, and the
`Priority` and `EventKind` enums, all with `serde` derives so they can be
serialized to JSON automatically.

Verify: `cargo build` with no warnings.

### Step 1.2 — Implement load and save

Still in `src/data.rs`, add two functions:
- `load(path) -> TrackerFile` — reads the JSON file; if the file doesn't exist,
  returns an empty `TrackerFile` (this is how `create` will bootstrap a new
  project)
- `save(path, &TrackerFile)` — writes the file, pretty-printed JSON

Verify: `cargo build` with no warnings.

### Step 1.3 — Smoke-test load/save from main

In `src/main.rs`, temporarily call `load("tracker.json")`, then `save` it back,
then print the result with `println!("{:#?}", ...)`. Run it and check that
`tracker.json` is created and is valid JSON.

```bash
cargo run
cat tracker.json
```

Delete `tracker.json` after confirming. Revert `main.rs` to a stub before the
next step.

---

## Phase 2: CLI Skeleton

Wire up `clap` with all the subcommands as stubs. None of them do real work yet
— they just print "not yet implemented" — but the argument shapes are all
declared so `--help` already shows the full interface.

### Step 2.1 — Define the CLI with clap

Create `src/cli.rs`. Use clap's `derive` API to define a `Cli` struct with a
`Commands` enum covering every subcommand: `create`, `list`, `show`, `status`,
`stuck`, `unstuck`, `blocked-by`, `unblock`, `label`, `delete`.

Include all flags and arguments each command will eventually accept (e.g.
`--priority`, `--label`, `--status` on `list`).

Verify:

```bash
cargo run -- --help
# Shows all subcommands

cargo run -- create --help
# Shows create's arguments
```

### Step 2.2 — Dispatch to stub handlers

In `src/main.rs`, match on the parsed command and call a stub function for each
one (e.g. `println!("create: not yet implemented")`).

Verify:

```bash
cargo run -- create "test issue"
# create: not yet implemented

cargo run -- list
# list: not yet implemented
```

---

## Phase 3: Core CRUD

Implement the commands one at a time. Each step adds one working command.

### Step 3.1 — `create`

Implement `create "title"`: load the tracker file, generate a new auto-
incremented ID, push a new `Issue` with an `opened` timeline event, save. Print
the new issue's ID.

Verify:

```bash
cargo run -- create "my first issue"
# Created issue #1: my first issue

cat tracker.json
# Valid JSON with one issue, opened event in timeline
```

### Step 3.2 — `list`

Implement `list` with no filters yet: load the tracker file, print all issues
that are not `done`. Format: one line per issue showing ID, title, and derived
status.

Verify:

```bash
cargo run -- create "second issue"
cargo run -- list
# #1  open  my first issue
# #2  open  second issue
```

### Step 3.3 — `show`

Implement `show <id>`: load, find the issue by ID (error if not found), print
all fields and the full timeline.

Verify:

```bash
cargo run -- show 1
# ID: 1
# Title: my first issue
# Status: open
# Timeline:
#   2024-05-21T10:30:00Z  opened
```

### Step 3.4 — `delete`

Implement `delete <id>`: load, remove the issue by ID (error if not found),
save. Print confirmation.

Verify:

```bash
cargo run -- delete 2
# Deleted issue #2

cargo run -- list
# #1  open  my first issue
```

### Step 3.5 — `status` (open / in-progress / done)

Implement `status <id> open|in-progress|done`: load, find the issue, validate
the transition (reject `done → anything`), append the appropriate timeline
event, save.

Verify:

```bash
cargo run -- status 1 in-progress
cargo run -- show 1
# Status: in-progress, timeline has two events

cargo run -- status 1 done
cargo run -- status 1 open
# error: issue #1 is done; done is a terminal state
```

---

## Phase 4: Priority

### Step 4.1 — Add priority to `create`

Wire up the `--priority low|medium|high` flag on `create`. Default to `medium`
if omitted. Store it on the issue. Print it in `list` and `show` output.

Verify:

```bash
cargo run -- create "urgent thing" --priority high
cargo run -- create "minor thing" --priority low
cargo run -- create "normal thing"
cargo run -- list
# Issues appear; check priority column is present
```

### Step 4.2 — Sort `list` by priority

Update `list` to sort by priority: `high` first, then `medium`, then `low`.

Verify:

```bash
cargo run -- list
# high issue appears before medium, medium before low
```

---

## Phase 5: Labels

### Step 5.1 — Add labels to `create`

Wire up `--label tag` (repeatable) on `create`. Lowercase and deduplicate on
write. Store as an array on the issue.

Verify:

```bash
cargo run -- create "login bug" --label bug --label auth --label Bug
cargo run -- show <id>
# labels: ["auth", "bug"]  ← lowercased, deduplicated
```

### Step 5.2 — `label` command

Implement `label <id> tag [tag...]`: load, find the issue, add label(s) (same
lowercase/dedup rules), save.

Verify:

```bash
cargo run -- label 1 feature cli
cargo run -- show 1
# labels include feature and cli
```

### Step 5.3 — `--label` filter on `list`

Implement `--label L` filtering on `list`. Multiple `--label` flags use AND
semantics (issue must have all specified labels).

Verify:

```bash
cargo run -- list --label bug
# Only issues with "bug" label

cargo run -- list --label bug --label auth
# Only issues with both "bug" AND "auth"

cargo run -- list --label nonexistent
# (empty output, no crash)
```

---

## Phase 6: Blocking Relationships

### Step 6.1 — `blocked-by` command

Implement `blocked-by <id> <other-id>`: load, validate both IDs exist, validate
no self-reference, add `other-id` to the issue's `blockedBy` array (dedup),
save.

Verify:

```bash
cargo run -- create "task A"
cargo run -- create "task B"
cargo run -- blocked-by 2 1
# issue #2 is now blocked by issue #1

cargo run -- show 2
# blockedBy: [1]
```

### Step 6.2 — Derive `blocked` status from `blockedBy`

Update status derivation: before checking the timeline, check if any issue in
`blockedBy` is not `done`. If so, status is `blocked`.

Verify:

```bash
cargo run -- list
# issue #2 shows as "blocked"

cargo run -- status 1 done
cargo run -- list
# issue #2 is no longer blocked (issue #1 is done)
```

### Step 6.3 — `unblock` command (remove dependency)

Implement `unblock <id> <other-id>`: remove `other-id` from the issue's
`blockedBy` array (error if the relationship doesn't exist).

Verify:

```bash
cargo run -- blocked-by 2 1
cargo run -- unblock 2 1
cargo run -- show 2
# blockedBy: []
```

### Step 6.4 — Orphan warning on load

Update the load function: after deserializing, check every `blockedBy` entry
against the set of known IDs. Print a warning to stderr for any that are missing
(do not modify the file).

Verify:

```bash
# Manually edit tracker.json and add a nonexistent ID to a blockedBy array
cargo run -- list
# warn: issue #2 references blockedBy [99] which does not exist — relationship ignored
# list still works
```

---

## Phase 7: Stuck / Unstuck Narrative

### Step 7.1 — `stuck` command

Implement `stuck <id> "reason"`: load, find the issue, validate it is not
already `stuck` (error if so), append a `stuck` timeline event with the reason
as a note, save.

Verify:

```bash
cargo run -- stuck 1 "unsure which approach to take"
cargo run -- show 1
# Latest timeline event: stuck — "unsure which approach to take"
cargo run -- list
# issue #1 shows as "stuck"

cargo run -- stuck 1 "another reason"
# error: issue #1 is already stuck
```

### Step 7.2 — `unstuck` command

Implement `unstuck <id> "what resolved it"`: validate the issue is currently
`stuck`, append an `unstuck` timeline event, save. Status falls back to the
previous non-stuck state.

Verify:

```bash
cargo run -- unstuck 1 "going with clap's derive API"
cargo run -- show 1
# Timeline shows stuck then unstuck events
cargo run -- list
# issue #1 is back to open or in-progress
```

---

## Phase 8: Compound Filters

### Step 8.1 — `--status` filter on `list`

Implement `--status open|in-progress|stuck|blocked|done` filtering on `list`.
When no `--status` flag is given, default to showing `open + in-progress + stuck
+ blocked` (i.e. everything not done).

Verify:

```bash
cargo run -- list --status blocked
cargo run -- list --status done
cargo run -- list --status open --label bug
# Combines correctly
```

### Step 8.2 — `--priority` filter on `list`

Implement `--priority low|medium|high` filtering on `list`. Priority sort still
applies within filtered results.

Verify:

```bash
cargo run -- list --priority high
cargo run -- list --priority high --label bug --status open
```

---

## Phase 9: Polish

### Step 9.1 — Input validation pass

Audit all commands for missing edge-case guards:
- Reject empty or whitespace-only titles on `create`
- Reject `stuck` with empty reason; `unstuck` with empty resolution
- Reject `blocked-by` self-reference
- Reject invalid status strings with a clear error message
- All commands that take an `<id>` should print a clear "issue not found" error

Verify each rejection with a bad input and confirm it exits non-zero without a
panic.

### Step 9.2 — Add colored output

Add `owo-colors` to `Cargo.toml`:

```toml
owo-colors = "3"
```

Color the list output: status labels get colors (`blocked`/`stuck` in yellow,
`done` in green, `in-progress` in cyan). Priority `high` in red. Timeline
event kinds in `show` get subtle coloring.

Verify visually in terminal.

### Step 9.3 — `--help` polish

Review all `clap` help strings for every subcommand and flag. Make sure they
match the final behavior (especially the `list` defaults and AND semantics for
`--label`).

Verify:

```bash
cargo run -- --help
cargo run -- list --help
cargo run -- blocked-by --help
```

### Step 9.4 — Final build and checklist

```bash
cargo build --release
```

Run through the verification checklist from DESIGN.md:
- `cargo build` compiles without warnings
- All commands work from the terminal
- JSON is valid and human-readable
- Edge cases don't crash
- List output is sorted correctly

The `--release` binary is at `target/release/tracker`. You can copy it anywhere
on your PATH if you want to use it outside the project directory.

---

## What's Left for a Future Session

Per DESIGN.md Out of Scope:
- Subissues / hierarchy
- Due dates
- Time tracking
- Database backend
- `pbjson` integration / explicit cross-compatibility
