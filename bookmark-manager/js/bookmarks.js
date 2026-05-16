import { getBookmarks } from './storage.js';

const template = document.getElementById('bookmark-item-template');

/**
 * Builds a single bookmark card DOM node from the template.
 * Uses textContent / setAttribute exclusively — no raw HTML interpolation.
 * @param {{ id: string, url: string, title: string, note?: string, tags?: string[] }} bookmark
 * @param {{ onEdit?: Function, onDelete?: Function }} callbacks
 * @returns {DocumentFragment}
 */
function createBookmarkElement(bookmark, { onEdit, onDelete } = {}) {
  const node = template.content.cloneNode(true);

  const titleEl = node.querySelector('.bookmark-title');
  titleEl.textContent = bookmark.title;
  titleEl.href = bookmark.url;

  const domainEl = node.querySelector('.bookmark-domain');
  try {
    const parsed = new URL(bookmark.url);
    domainEl.textContent = parsed.hostname;
  } catch {
    domainEl.textContent = bookmark.url;
  }

  if (bookmark.note) {
    const noteEl = node.querySelector('.bookmark-note');
    noteEl.textContent = bookmark.note;
    noteEl.classList.remove('hidden');
  }

  if (bookmark.tags && bookmark.tags.length > 0) {
    const tagsEl = node.querySelector('.bookmark-tags');
    tagsEl.classList.remove('hidden');
    for (const tag of bookmark.tags) {
      const badge = document.createElement('span');
      badge.className = 'tag-badge';
      badge.textContent = tag;
      tagsEl.appendChild(badge);
    }
  }

  node.querySelector('.edit-btn').addEventListener('click', () => onEdit?.(bookmark));
  node.querySelector('.delete-btn').addEventListener('click', () => onDelete?.(bookmark.id));

  return node;
}

/**
 * Reads bookmarks from storage, applies tag filters (AND) then search query, and re-renders.
 * Tag filters are applied first; search runs within those results.
 * @param {{ onEdit?: Function, onDelete?: Function, activeTags?: string[], searchQuery?: string }} options
 */
export function renderBookmarks({ onEdit, onDelete, activeTags = [], searchQuery = '' } = {}) {
  const list = document.getElementById('bookmark-list');
  const emptyState = document.getElementById('empty-state');
  const allBookmarks = getBookmarks();
  let bookmarks = allBookmarks;

  if (activeTags.length > 0) {
    bookmarks = bookmarks.filter((b) => activeTags.every((tag) => b.tags?.includes(tag)));
  }

  if (searchQuery) {
    const q = searchQuery.toLowerCase();
    bookmarks = bookmarks.filter((b) =>
      b.title.toLowerCase().includes(q) ||
      (b.note && b.note.toLowerCase().includes(q))
    );
  }

  list.replaceChildren();

  if (bookmarks.length === 0) {
    emptyState.textContent = emptyStateMessage(allBookmarks.length, activeTags, searchQuery);
    emptyState.classList.remove('hidden');
    return;
  }

  emptyState.classList.add('hidden');
  for (const bookmark of bookmarks) {
    list.appendChild(createBookmarkElement(bookmark, { onEdit, onDelete }));
  }
}

/**
 * Returns an appropriate empty-state message based on the current filter state.
 * @param {number} totalCount
 * @param {string[]} activeTags
 * @param {string} searchQuery
 * @returns {string}
 */
function emptyStateMessage(totalCount, activeTags, searchQuery) {
  if (totalCount === 0) {
    return 'No bookmarks yet. Add one above!';
  }
  const tagList = activeTags.map((t) => `"${t}"`).join(' + ');
  if (activeTags.length > 0 && searchQuery) {
    return `No bookmarks tagged ${tagList} match "${searchQuery}".`;
  }
  if (searchQuery) {
    return `No bookmarks match "${searchQuery}".`;
  }
  return `No bookmarks tagged ${tagList}.`;
}
