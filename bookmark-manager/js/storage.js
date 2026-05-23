const STORAGE_KEY = 'bookmarks';

// In-memory cache. Populated on first read, invalidated on every write so
// multiple getBookmarks() calls within the same refresh cycle share one parse.
let cache = null;

/**
 * Returns the stored bookmark array, or [] on failure.
 * Subsequent calls within the same event loop tick return the cached value
 * without re-parsing localStorage.
 * @returns {object[]}
 */
export function getBookmarks() {
  if (cache !== null) return cache;
  try {
    cache = JSON.parse(localStorage.getItem(STORAGE_KEY)) ?? [];
  } catch {
    cache = [];
  }
  return cache;
}

/**
 * Overwrites the stored bookmark array and updates the cache.
 * @param {object[]} bookmarks
 */
export function saveBookmarks(bookmarks) {
  cache = bookmarks;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(bookmarks));
}

/**
 * Prepends a new bookmark to the list and persists it.
 * @param {object} bookmark
 */
export function addBookmark(bookmark) {
  const bookmarks = getBookmarks();
  bookmarks.unshift(bookmark);
  saveBookmarks(bookmarks);
}

/**
 * Merges changes into the bookmark with the given id.
 * @param {string} id
 * @param {object} changes
 */
export function updateBookmark(id, changes) {
  const bookmarks = getBookmarks();
  const idx = bookmarks.findIndex((b) => b.id === id);
  if (idx === -1) return;
  bookmarks[idx] = { ...bookmarks[idx], ...changes };
  saveBookmarks(bookmarks);
}

/**
 * Removes the bookmark with the given id.
 * @param {string} id
 */
export function deleteBookmark(id) {
  saveBookmarks(getBookmarks().filter((b) => b.id !== id));
}

/**
 * Returns the first bookmark whose stored URL matches the given URL, or null.
 * @param {string} url
 * @returns {object | null}
 */
export function findBookmarkByUrl(url) {
  return getBookmarks().find((b) => b.url === url) ?? null;
}
