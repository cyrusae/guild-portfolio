const STORAGE_KEY = 'bookmarks';

/**
 * Returns the stored bookmark array, or [] on failure.
 * @returns {object[]}
 */
export function getBookmarks() {
  try {
    return JSON.parse(localStorage.getItem(STORAGE_KEY)) ?? [];
  } catch {
    return [];
  }
}

/**
 * Overwrites the stored bookmark array.
 * @param {object[]} bookmarks
 */
export function saveBookmarks(bookmarks) {
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
