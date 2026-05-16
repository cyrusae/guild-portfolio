import { renderBookmarks } from './bookmarks.js';
import { initModal, openEditModal } from './modal.js';
import { deleteBookmark } from './storage.js';

document.addEventListener('DOMContentLoaded', () => {
  function refresh() {
    renderBookmarks({ onEdit, onDelete });
  }

  function onEdit(bookmark) {
    openEditModal(bookmark, refresh);
  }

  function onDelete(id) {
    if (!confirm('Delete this bookmark?')) return;
    deleteBookmark(id);
    refresh();
  }

  initModal(refresh);
  refresh();
});
