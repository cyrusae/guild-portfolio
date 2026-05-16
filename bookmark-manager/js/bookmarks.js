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
 * Reads bookmarks from storage and re-renders the list.
 * Toggles the empty state message as needed.
 * @param {{ onEdit?: Function, onDelete?: Function }} callbacks
 */
export function renderBookmarks({ onEdit, onDelete } = {}) {
  const list = document.getElementById('bookmark-list');
  const emptyState = document.getElementById('empty-state');
  const bookmarks = getBookmarks();

  list.replaceChildren();

  if (bookmarks.length === 0) {
    emptyState.classList.remove('hidden');
    return;
  }

  emptyState.classList.add('hidden');
  for (const bookmark of bookmarks) {
    list.appendChild(createBookmarkElement(bookmark, { onEdit, onDelete }));
  }
}
