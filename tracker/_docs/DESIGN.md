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
          "event": "blocked",
          "note": "Unsure which color library to use"
        },
        {
          "timestamp": "2024-05-21T14:00:00Z",
          "event": "unblocked",
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
- **labels**: Array of tags for categorization (e.g., `["bug", "feature", "cli"]`)
- **blockedBy**: Array of issue IDs this issue is blocked by
- **timeline**: Immutable log of state transitions with optional context notes

### Status Model

Status is derived from the timeline (current event is the last one):

- `opened`: Most recent event is `opened`
- `in-progress`: Most recent event is `in-progress`
- `blocked`: Most recent event is `blocked`
- `done`: Most recent event is `closed`

## CLI Interface

```bash
tracker create "title" [--priority low|medium|high] [--label tag] [--label tag...]
  Create a new issue

tracker list [--priority P] [--label L] [--status open|in-progress|blocked|done]
  List issues, sorted by priority (high → low)
  Default: show open + in-progress, grouped by status

tracker show <id>
  Show full details including timeline

tracker status <id> open|in-progress|done
  Change status (opens/closes an issue, or marks in-progress)

tracker block <id> "reason for blockage"
  Mark as blocked with context note

tracker unblock <id> "what resolved it"
  Mark as unblocked with resolution note

tracker label <id> tag [tag...]
  Add label(s) to an issue

tracker delete <id>
  Remove an issue entirely

tracker --help
  Show help for all commands
```

## Why This Design

- **Timeline as source of truth**: No separate state field. The last event in the timeline IS the current state. This is immutable and uploadable.
- **Blocked/unblocked events are first-class**: Captures the stuck/unstuck narrative that matters for context. When you upload to Claude, it sees "I was blocked on X, I learned Y, here's where I am."
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
4. **Blocking**: `blockedBy` field and blocking relationships. Show blocked status separately.
5. **Timeline**: Implement timeline events (opened, in-progress, blocked, unblocked, closed). Derive current status from timeline.
6. **Compound filters**: Status + priority + label filters work together.
7. **Polish**: Error messages, colored output, additional edge case handling, `--helop` output.

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
