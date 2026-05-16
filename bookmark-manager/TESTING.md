# Test cases

## Unit Test Cases (Manual / Browser Console)

### Edge Cases: URL Input

- [ ]  Empty URL (should reject)
- [ ]  URL without protocol (e.g., "example.com" → accept and prepend https://)
- [ ]  URL with spaces (trim and accept)
- [ ]  Invalid URL (no domain, e.g., "https://") (reject)
- [ ]  URL with query params / fragments (accept as-is)
- [ ]  Very long URL (accept; handle gracefully in display)
- [ ]  URL with special characters (should be escaped/sanitized)

### Edge Cases: Title

- [ ]  Empty title (allow; use URL as fallback display)
- [ ]  Very long title (truncate or wrap gracefully)
- [ ]  Title with HTML-like tags (e.g., `<script>`)--sanitize/escape

### Edge Cases: Notes

- [ ]  No notes (optional; should work)
- [ ]  Notes with newlines (preserve)
- [ ]  Notes with HTML (sanitize)
- [ ]  Very long notes (display truncated, show "..." + expand on hover/click)

### Edge Cases: Tags

- [ ]  No tags (should work; bookmark still saves)
- [ ]  Empty tag input (don't create empty tags)
- [ ]  Tags with spaces (trim; split on commas)
- [ ]  Duplicate tags in input (e.g., "python, python" → deduplicate to ["python"])
- [ ]  Tags with special characters (sanitize or reject)

### Duplicate Detection

- [ ]  Save bookmark with URL X (succeeds)
- [ ]  Save another with same URL X (rejected; shows toast "Already bookmarked")
- [ ]  Normalize URLs: "[https://example.com](https://example.com/)" vs "[https://example.com/](https://example.com/)" (treat as duplicate)
- [ ]  Normalize: "Example.com" vs "example.com" (case-insensitive domain)

### Display & Filtering

- [ ]  No bookmarks (show empty state)
- [ ]  One bookmark (display correctly)
- [ ]  Many bookmarks (display in reverse chronological order; performance acceptable?)
- [ ]  Filter by tag with no matches (show "No bookmarks with tag #X")
- [ ]  Filter by tag with matches (show only those)
- [ ]  Search with no matches (show "No results for 'X'")
- [ ]  Search with matches (highlight or show only matches)
- [ ]  Combine tag filter + search (show intersection, AND logic)

### Delete

- [ ]  Delete a bookmark (removed from display, removed from localStorage)
- [ ]  Confirm dialog before delete (prevents accidents)

### Sanitization / Security

- [ ]  Inject `<img src=x onerror="alert('XSS')">` in title (should be escaped, not executed)
- [ ] Inject `<script>alert('XSS')</script>` in notes (should be escaped)
- [ ] Inject `javascript:alert('XSS')` as URL (reject)

**Do** suggest other test/edge cases if you think of them while building.