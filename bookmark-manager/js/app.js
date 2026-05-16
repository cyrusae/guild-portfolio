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
    renderBookmarks({ onEdit, onDelete, onTagClick, activeTags, searchQuery });
  }

  function onEdit(bookmark) {
    openEditModal(bookmark, refresh);
  }

  function onTagClick(tag) {
    if (!activeTags.includes(tag)) {
      activeTags = [...activeTags, tag];
      refresh();
    }
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

  // On touch devices, tapping outside any card dismisses the focused card.
  document.addEventListener('click', () => {
    document.querySelectorAll('.bookmark-item.is-focused')
      .forEach((el) => el.classList.remove('is-focused'));
  });

  initModal(refresh);
  refresh();
});
