# Issue Tracker CLI - Specification

## Purpose

A personal issue tracker CLI that captures not just what you're building, but where you got stuck and how you got unstuck. Designed to be:

- **Uploadable**: Single JSON file snapshots to Claude for context
- **Agent-readable**: Blocking notes capture decision context and research done; agents can naturally use issue tracker to take notes over the course of a conversation
- **Atomic**: Issues are the smallest unit you want to remember and check off
- **Traceable**: Timeline of state transitions preserves your workflow narrative

## Data Model

Single JSON file (`tracker.json` or similar) in project directory with flat structure:

```json
{
  "meta": {
    "name": "project-name",
    "createdAt": "2024-05-21T10:00:00Z"
  },
  "issues": [
    {
      "id": "1",
      "title": "Add colored output",
      "priority": "medium",
      "labels": ["feature", "cli"],
      "blockedBy": [],
      "timeline": [
        {
          "timestamp": "2024-05-21T10:30:00Z",
          "event": "opened"
        },
        {
          "timestamp": "2024-05-21T11:00:00Z",
          "event": "stuck",
          "note": "Unsure which color library to use"
        },
        {
          "timestamp": "2024-05-21T14:00:00Z",
          "event": "unstuck",
          "note": "crossterm is standard for TUI work in Rust"
        },
        {
          "timestamp": "2024-05-21T15:00:00Z",
          "event": "closed"
        }
      ]
    }
  ]
}
```

### Field Details

- **id**: Unique identifier (integer or UUID, auto-incremented)
- **title**: Atomic unit of work. "Colored output" not "Make the CLI better"
- **priority**: `low`, `medium`, or `high`
- **labels**: Array of tags for categorization (e.g., `["bug", "feature", "cli"]`). Stored lowercase; duplicates silently deduplicated on write.
- **blockedBy**: Array of issue IDs this issue depends on. An issue is `blocked` if any referenced issue is not `done`.
- **timeline**: Immutable append-only log of state transitions with optional context notes.

### Status Model

Status is derived from two sources, with `blockedBy` taking precedence:

1. If any issue in `blockedBy` is not `done` → status is `blocked`
2. Otherwise, status comes from the most recent timeline event:
   - `opened` → `open`
   - `in-progress` → `in-progress`
   - `stuck` → `stuck`
   - `unstuck` → falls back to previous non-stuck/non-unstuck state (effectively `in-progress` or `open`)
   - `closed` → `done`

**State machine rules:**
- `done` is terminal. No transitions out of `done` are allowed.
- You cannot add a `stuck` event to an issue that is already `stuck` (rejected with an error).
- `stuck`/`unstuck` are internal narrative events; `blocked` status is derived from `blockedBy` relationships only.

**Orphan handling:** On load, if a `blockedBy` entry references an issue ID that does not exist, a warning is emitted (`warn: issue <id> references blockedBy [<missing-id>] which does not exist — relationship ignored`) and the reference is ignored for status purposes. The file is not modified.

## CLI Interface

```bash
tracker create "title" [--priority low|medium|high] [--label tag] [--label tag...]
  Create a new issue

tracker list [--priority P] [--label L] [--status open|in-progress|stuck|blocked|done]
  List issues, sorted by priority (high → low)
  Default: show open + in-progress + stuck + blocked, grouped by status
  Multiple --label flags use AND semantics (issue must have all specified labels)

tracker show <id>
  Show full details including timeline

tracker status <id> open|in-progress|done
  Change status (opens/closes an issue, or marks in-progress)
  Note: done is terminal; transitions out of done are not allowed

tracker stuck <id> "reason"
  Record an internal blockage note on the timeline (you hit a wall within this issue)

tracker unstuck <id> "what resolved it"
  Record the resolution of an internal blockage on the timeline

tracker blocked-by <id> <other-id>
  Add a dependency: this issue is blocked until <other-id> is done

tracker unblock <id> <other-id>
  Remove a dependency relationship between two issues

tracker label <id> tag [tag...]
  Add label(s) to an issue (labels are lowercased; duplicates ignored)

tracker delete <id>
  Remove an issue entirely

tracker --help
  Show help for all commands
```

## Why This Design

- **Timeline as source of truth**: No separate state field. Status is derived from the timeline and `blockedBy` relationships. The file is append-only by design and uploadable as a snapshot.
- **Two kinds of blockage**: `stuck`/`unstuck` are narrative timeline events (internal — you hit a wall within this issue's scope). `blocked` status is derived from `blockedBy` relationships (external — another issue is in the way). These are kept separate so the upload context is always clear: "I was stuck on X within this issue" vs. "issue Y must be done first."
- **Flat structure**: No subissues or hierarchy. Phase 2 scope is atomic units + blocking. Hierarchy is Phase 2.5.
- **Priority + labels**: Required by curriculum. Enables filtering and sorting.
- **Single file**: Everything is in one JSON. Portable, uploadable, snapshotable.

## Out of Scope (Phase 2)

- Subissues or hierarchy
- Due dates or calendar integration
- Time tracking
- Persistent blocking reasons (why are you blocked, not just that you are)
- Database backend (JSON only)
- Man page generation (--help is enough)
- Subprojects

## Build Steps

1. **Core**: Create, list, show, delete. Open/done status. JSON storage.
2. **Priority**: Add priority levels. Sort by priority in list view.
3. **Labels**: Add label support. Filter by label(s).
4. **Blocking**: `blockedBy` field and `blocked-by`/`unblock` commands. Derive `blocked` status from unresolved dependencies. Warn on orphaned references at load time.
5. **Timeline**: Implement `stuck`/`unstuck` timeline events alongside `opened`, `in-progress`, `closed`. Derive current status from timeline + `blockedBy`.
6. **Compound filters**: Status + priority + label filters work together.
7. **Polish**: Error messages, colored output, additional edge case handling, `--help` output.

## Security Considerations

- Validate all input from command line (reject empty titles, invalid IDs)
- Handle missing or corrupted JSON file gracefully (don't crash)
- When reading/writing JSON, handle file I/O errors
- Sanitize user input before serializing to JSON

## Verification Checklist

When each layer is done:

- `cargo build` compiles without warnings
- All commands in that layer work from the terminal
- JSON is valid and human-readable
- Edge cases don't crash (missing file, invalid ID, empty list, etc.)
- List output is sorted correctly for that layer
