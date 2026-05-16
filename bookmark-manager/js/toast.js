const TOAST_DURATION_MS = 3500;
const TOAST_ACTION_DURATION_MS = 6000;

/**
 * Displays a brief toast notification.
 * @param {string} message
 * @param {'success' | 'error'} type
 * @param {{ label: string, onClick: () => void } | null} action - optional action button
 */
export function showToast(message, type = 'success', action = null) {
  const container = document.getElementById('toast-container');

  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;

  const text = document.createElement('span');
  text.textContent = message;
  toast.appendChild(text);

  if (action) {
    const btn = document.createElement('button');
    btn.className = 'toast-action';
    btn.textContent = action.label;
    btn.addEventListener('click', () => {
      toast.remove();
      action.onClick();
    });
    toast.appendChild(btn);
  }

  container.appendChild(toast);
  setTimeout(() => toast.remove(), action ? TOAST_ACTION_DURATION_MS : TOAST_DURATION_MS);
}
