import { getBookmarks } from './storage.js';

/**
 * Returns a sorted array of every unique tag across all stored bookmarks.
 * @returns {string[]}
 */
export function getUniqueTags() {
  const bookmarks = getBookmarks();
  return [...new Set(bookmarks.flatMap((b) => b.tags ?? []))].sort();
}

/**
 * Renders the tag filter bar.
 * Shows an "All" button plus one button per unique tag.
 * Hides the container entirely when no tags exist.
 * @param {{ activeTag: string | null, onSelect: (tag: string | null) => void }} options
 */
export function renderTagFilters({ activeTag, onSelect }) {
  const container = document.getElementById('tag-filters');
  const tags = getUniqueTags();

  container.replaceChildren();

  if (tags.length === 0) {
    container.classList.add('hidden');
    return;
  }

  container.classList.remove('hidden');

  const allBtn = document.createElement('button');
  allBtn.className = activeTag === null ? 'tag-filter-btn active' : 'tag-filter-btn';
  allBtn.textContent = 'All';
  allBtn.addEventListener('click', () => onSelect(null));
  container.appendChild(allBtn);

  for (const tag of tags) {
    const btn = document.createElement('button');
    btn.className = tag === activeTag ? 'tag-filter-btn active' : 'tag-filter-btn';
    btn.textContent = tag;
    btn.addEventListener('click', () => onSelect(tag));
    container.appendChild(btn);
  }
}
