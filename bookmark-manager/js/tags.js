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
 * Supports multi-select: clicking an inactive tag adds it to the active set;
 * clicking an active tag removes it. "All" clears the selection.
 * Hides the container entirely when no tags exist.
 * @param {{ activeTags: string[], onSelect: (tags: string[]) => void }} options
 */
export function renderTagFilters({ activeTags, onSelect }) {
  const container = document.getElementById('tag-filters');
  const tags = getUniqueTags();

  container.replaceChildren();

  if (tags.length === 0) {
    container.classList.add('hidden');
    return;
  }

  container.classList.remove('hidden');

  const allBtn = document.createElement('button');
  allBtn.className = activeTags.length === 0 ? 'tag-filter-btn active' : 'tag-filter-btn';
  allBtn.textContent = 'All';
  allBtn.addEventListener('click', () => onSelect([]));
  container.appendChild(allBtn);

  for (const tag of tags) {
    const isActive = activeTags.includes(tag);
    const btn = document.createElement('button');
    btn.className = isActive ? 'tag-filter-btn active' : 'tag-filter-btn';
    btn.textContent = tag;
    btn.addEventListener('click', () => {
      onSelect(isActive
        ? activeTags.filter((t) => t !== tag)
        : [...activeTags, tag]
      );
    });
    container.appendChild(btn);
  }
}
