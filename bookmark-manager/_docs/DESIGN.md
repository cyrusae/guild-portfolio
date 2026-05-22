# Toy bookmark manager

**Language:** Vanilla JS/HTML/CSS.
**Output:** Single-page browser app.
**Target:** Desktop and mobile browsers (should handle mobile screen size gracefully).
**Goal:** Single-user bookmark manager using browser storage; iteratively add features (tags, notes, filtering, duplicate detection, editing/deleting, search).

We will work through this document in order: Core -> Additional features (one by one!) -> Enhancements -> Style. Divide tasks into small chunks and pause frequently, don't try to rush through things within a single turn's token limit. Stop and verify whenever you finish a step. When in doubt, talk to me.

## Core

1. Display a list of bookmarks or an empty page 
2. Add a new bookmark (URL and title only) in a modal popup
3. Bookmarks persist in browser storage

### Next steps:

- URLs should be validated--must have a domain; must start with `http://` or `https://`--and trim tracker junk and whitespaces
- Confirmation toast when a bookmark is successfully added
- Start sanitizing inputs! I'm not an attacker but you have no business trusting that

## Additional features

We will work through these one by one, in order.

- **Addition #1:** *Add two fields to the add form: an optional note (a text area for a brief description) and optional tags (comma-separated text that gets stored as a list). Display the note under each bookmark's title, and display tags as small labeled badges.*
- **Addition #2:** *Each bookmark should have small edit and delete icons. Clicking edit opens a modal form (same as the add bookmark modal) pre-filled with the bookmark's current data. You can modify title, URL, note, and tags. Save persists changes; cancel closes without saving. Clicking delete removes the bookmark after a confirmation.*
- **Addition #3:** *Above the bookmark list, display all unique tags as clickable buttons. When I click a tag, the list should filter to only show bookmarks with that tag. There should be an 'All' button that removes the filter. The currently active filter should be visually highlighted.*
- **Addition #4:** *Add a search bar next to the tag filters. It should filter the bookmark list in real time as I type, matching against bookmark titles and notes. Search and tag filters should work together. If I have a tag filter active and then search, it should search within the filtered results.* Search should be case-insensitive.

### Enhancements

- Duplicate detection: trim whitespace and add https:// protocol if needed, then hash URLs and alert if one has already been bookmarked before. If a bookmark with that URL already exists, **don't insert**; instead, show a toast notification: "Bookmark already exists. Update it instead?" with a link to that bookmark
- Improve URL validation: accept links without `https` prefix, add it automatically.
- Allow filtering by multiple tags (`AND`)

### Visual style

**Color scheme:** Catppuccin Mocha 
**Aesthetic:** Cyberpunk terminal--blocky squared-off borders, "hacker" aesthetic, monospace Nerd Font
**Screen sizes:** Desktop and mobile support

### Testing

Use `TESTING.md` for suggestions for edge cases/verification handling *after* implementing the core, addition, and enhancement features.

## Code style

Starting point: `index.html`. However: **Code belongs in language-specific files.** If an IDE wouldn't know what language to highlight a block of code in, refactor your code.

### Specific rules

- **Never use `<style>` blocks in `.js` files**
    - Extract all styles to corresponding `.css` files
    - WRONG: `script.js` with `<style>` tag
    - RIGHT: `component.js`, `component.css`, import in component.js
- **Never use inline `style` attributes for component styling**
    - WRONG: `<div style="color: red; padding: 10px;">text</div>`
    - RIGHT: Add class, define in `.css` file
- **Import at the top of `.js` files** (no lazy-loading)
    - Explicit imports act as self-documentation
    - Readers know immediately what a module depends on
- **Template strings in `.js`: Single source line only**
	- WRONG: Multi-line template string in `.js`
	- RIGHT: Multi-line markup → separate `.html` file and import
