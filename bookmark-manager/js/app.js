import { renderBookmarks } from './bookmarks.js';
import { initModal, openEditModal } from './modal.js';
import { deleteBookmark } from './storage.js';
import { renderTagFilters } from './tags.js';

document.addEventListener('DOMContentLoaded', () => {
  let activeTag = null;
  let searchQuery = '';

  function refresh() {
    renderTagFilters({
      activeTag,
      onSelect: (tag) => {
        activeTag = tag;
        refresh();
      },
    });
    renderBookmarks({ onEdit, onDelete, activeTag, searchQuery });
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
