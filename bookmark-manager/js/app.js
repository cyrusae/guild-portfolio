import { renderBookmarks } from './bookmarks.js';
import { initModal, openEditModal } from './modal.js';
import { deleteBookmark } from './storage.js';
import { renderTagFilters, getUniqueTags } from './tags.js';

document.addEventListener('DOMContentLoaded', () => {
  let activeTags = [];
  let searchQuery = '';

  function refresh() {
    // Drop any selected tags that no longer exist (e.g. after editing a bookmark).
    const existingTags = getUniqueTags();
    activeTags = activeTags.filter((t) => existingTags.includes(t));

    renderTagFilters({
      activeTags,
      onSelect: (tags) => {
        activeTags = tags;
        refresh();
      },
    });
    renderBookmarks({ onEdit, onDelete, activeTags, searchQuery });
  }

  function onEdit(bookmark) {
    openEditModal(bookmark, refresh);
  }

  function onDelete(id) {
    if (!confirm('Delete this bookmark?')) return;
    deleteBookmark(id);
    refresh();
  }

  document.getElementById('search-input').addEventListener('input', (e) => {
    searchQuery = e.target.value.trim();
    refresh();
  });

  initModal(refresh);
  refresh();
});
