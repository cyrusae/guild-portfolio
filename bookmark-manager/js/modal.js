import { validateUrl, cleanUrl } from './url.js';
import { addBookmark, updateBookmark } from './storage.js';
import { showToast } from './toast.js';

const overlay = document.getElementById('modal-overlay');
const modalTitleEl = document.getElementById('modal-title');
const form = document.getElementById('add-bookmark-form');
const urlInput = document.getElementById('input-url');
const titleInput = document.getElementById('input-title');
const noteInput = document.getElementById('input-note');
const tagsInput = document.getElementById('input-tags');
const urlError = document.getElementById('url-error');
const titleError = document.getElementById('title-error');

// Tracks the id of the bookmark being edited, or null in add mode.
let editingId = null;
// Called after a successful save so the caller can re-render.
let onSavedCallback = null;

function openModal() {
  editingId = null;
  modalTitleEl.textContent = 'Add Bookmark';
  overlay.classList.remove('hidden');
  urlInput.focus();
}

function closeModal() {
  overlay.classList.add('hidden');
  editingId = null;
  form.reset();
  clearErrors();
}

function clearErrors() {
  urlInput.classList.remove('invalid');
  titleInput.classList.remove('invalid');
  urlError.textContent = '';
  urlError.classList.add('hidden');
  titleError.textContent = '';
  titleError.classList.add('hidden');
}

function showFieldError(inputEl, errorEl, message) {
  inputEl.classList.add('invalid');
  errorEl.textContent = message;
  errorEl.classList.remove('hidden');
}

function handleSubmit(e) {
  e.preventDefault();
  clearErrors();

  const rawUrl = urlInput.value;
  const rawTitle = titleInput.value.trim();
  let hasError = false;

  const urlResult = validateUrl(rawUrl);
  if (!urlResult.valid) {
    showFieldError(urlInput, urlError, urlResult.error);
    hasError = true;
  }

  if (!rawTitle) {
    showFieldError(titleInput, titleError, 'Title is required.');
    hasError = true;
  }

  if (hasError) {
    return;
  }

  const note = noteInput.value.trim();
  const tags = tagsInput.value
    .split(',')
    .map((t) => t.trim())
    .filter(Boolean);
  const url = cleanUrl(urlResult.url);

  const isEditing = editingId !== null;

  if (isEditing) {
    updateBookmark(editingId, { url, title: rawTitle, note, tags });
  } else {
    addBookmark({
      id: crypto.randomUUID(),
      url,
      title: rawTitle,
      note,
      tags,
      createdAt: new Date().toISOString(),
    });
  }

  onSavedCallback?.();
  closeModal();
  showToast(isEditing ? 'Bookmark updated!' : 'Bookmark saved!');
}

/**
 * Wires up all modal event listeners. Call once on DOMContentLoaded.
 * @param {Function} onSaved - called after a successful add or edit
 */
export function initModal(onSaved) {
  onSavedCallback = onSaved;

  document.getElementById('open-modal-btn').addEventListener('click', openModal);
  document.getElementById('modal-close-btn').addEventListener('click', closeModal);
  document.getElementById('modal-cancel-btn').addEventListener('click', closeModal);

  overlay.addEventListener('click', (e) => {
    if (e.target === overlay) closeModal();
  });

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && !overlay.classList.contains('hidden')) {
      closeModal();
    }
  });

  form.addEventListener('submit', handleSubmit);
}

/**
 * Opens the modal pre-filled with an existing bookmark's data.
 * @param {{ id: string, url: string, title: string, note?: string, tags?: string[] }} bookmark
 * @param {Function} onSaved - called after a successful save
 */
export function openEditModal(bookmark, onSaved) {
  onSavedCallback = onSaved;
  editingId = bookmark.id;
  modalTitleEl.textContent = 'Edit Bookmark';

  urlInput.value = bookmark.url;
  titleInput.value = bookmark.title;
  noteInput.value = bookmark.note ?? '';
  tagsInput.value = (bookmark.tags ?? []).join(', ');

  clearErrors();
  overlay.classList.remove('hidden');
  titleInput.focus();
}
