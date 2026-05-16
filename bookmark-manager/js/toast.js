const TOAST_DURATION_MS = 3500;

/**
 * Displays a brief toast notification.
 * @param {string} message
 * @param {'success' | 'error'} type
 */
export function showToast(message, type = 'success') {
  const container = document.getElementById('toast-container');

  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;
  toast.textContent = message;

  container.appendChild(toast);
  setTimeout(() => toast.remove(), TOAST_DURATION_MS);
}
