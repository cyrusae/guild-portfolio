# Bookmark manager example project

Toy bookmark manager made with Claude Code for the Navigators Guild. Intentionally minimal in-browser storage, Catppuccin Mocha cyberpunk layout, notes and tagging functionality.

This project is meant for educational use only and is not appropriate for personal use/should not be taken as "real" work. It is an experiment in defining processes for agentic development.

See `_docs` for more:

- [`DESIGN.md`](_docs/DESIGN.md) for the initial design document given to Claude Code
- [`PROCESS.md`](_docs/PROCESS.md) for a description of the process overall
- [`TESTING.md`](_docs/TESTING.md) for specified edge cases, exceptions, and breaking points to consider
- [`FEEDBACK.md`](_docs/FEEDBACK.md) for generated adversarial feedback on the initial build

---

## Project structure

```
bookmark-manager/
│
├── index.html              # Single page — markup, <template> for bookmark cards, script entry
│
├── js/
│   ├── app.js              # Entry point: wires modules together, owns app state (activeTags, searchQuery), drives refresh()
│   ├── storage.js          # localStorage CRUD with in-memory cache; getBookmarks / saveBookmarks / add / update / delete
│   ├── bookmarks.js        # renderBookmarks() + createBookmarkElement(); handles filtering, search, and card DOM
│   ├── tags.js             # getUniqueTags() and renderTagFilters(); tag filter bar UI
│   ├── modal.js            # Add / edit bookmark modal: open, close, form validation, submit
│   ├── toast.js            # Ephemeral toast notification helper
│   └── url.js              # URL normalisation and protocol whitelist validation
│
├── styles/
│   ├── main.css            # CSS custom properties (Catppuccin Mocha palette), resets, layout
│   ├── bookmarks.css       # Bookmark card, hover/focus action reveal, note clamp + toggle
│   ├── modal.css           # Modal overlay and form styles
│   ├── search.css          # Search input styles
│   ├── tags.css            # Tag filter bar and badge styles
│   └── toast.css           # Toast notification styles
│
└── _docs/
    ├── DESIGN.md           # Original design brief
    ├── PROCESS.md          # Agentic development process notes
    ├── TESTING.md          # Manual test cases and edge cases
    └── FEEDBACK.md         # Adversarial code review findings
```
