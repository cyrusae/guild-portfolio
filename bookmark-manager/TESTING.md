# Test cases

## Unit Test Cases (Manual / Browser Console)

### Edge Cases: URL Input

- [x]  Empty URL (should reject)
- [x]  URL without protocol (e.g., "example.com" → accept and prepend https://)
- [x]  URL with spaces (trim and accept)
- [x]  Invalid URL (no domain, e.g., "https://") (reject)
- [x]  URL with query params / fragments (accept as-is)
- [x]  Very long URL (accept; handle gracefully in display)
- [x]  URL with special characters (should be escaped/sanitized)

### Edge Cases: Title

- [ ]  Empty title (allow; use URL as fallback display)
- [x]  Very long title (truncate or wrap gracefully)
- [x]  Title with HTML-like tags (e.g., `<script>`)--sanitize/escape

### Edge Cases: Notes

- [x]  No notes (optional; should work)
- [x]  Notes with newlines (preserve)
- [x]  Notes with HTML (sanitize)
- [ ]  Very long notes (display truncated, show "..." + expand on hover/click)

### Edge Cases: Tags

- [x]  No tags (should work; bookmark still saves)
- [x]  Empty tag input (don't create empty tags)
- [x]  Tags with spaces (trim; split on commas)
- [x]  Duplicate tags in input (e.g., "python, python" → deduplicate to ["python"])
- [x]  Tags with special characters (sanitize or reject)

### Duplicate Detection

- [x]  Save bookmark with URL X (succeeds)
- [x]  Save another with same URL X (rejected; shows toast "Already bookmarked")
- [x]  Normalize URLs: "[https://example.com](https://example.com/)" vs "[https://example.com/](https://example.com/)" (treat as duplicate)
- [x]  Normalize: "Example.com" vs "example.com" (case-insensitive domain)

### Display & Filtering

- [x]  No bookmarks (show empty state)
- [x]  One bookmark (display correctly)
- [x]  Many bookmarks (display in reverse chronological order; performance acceptable?)
- [x]  Filter by tag with no matches (show "No bookmarks with tag #X")
- [x]  Filter by tag with matches (show only those)
- [x]  Search with no matches (show "No results for 'X'")
- [x]  Search with matches (highlight or show only matches)
- [x]  Combine tag filter + search (show intersection, AND logic)

### Delete

- [x]  Delete a bookmark (removed from display, removed from localStorage)
- [x]  Confirm dialog before delete (prevents accidents)

### Sanitization / Security

- [x] Inject `<img src=x onerror="alert('XSS')">` in title (should be escaped, not executed)
- [x] Inject `<script>alert('XSS')</script>` in notes (should be escaped)
- [x] Inject `javascript:alert('XSS')` as URL (reject)

**Do** suggest other test/edge cases if you think of them while building.
