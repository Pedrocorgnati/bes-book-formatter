/**
 * Centralized keyboard shortcuts handler.
 * Ignores events when focus is in INPUT/TEXTAREA/SELECT.
 */

interface Shortcut {
  key: string;
  meta?: boolean;  // Cmd (Mac) / Ctrl (Windows/Linux)
  shift?: boolean;
  action: () => void;
}

export function registerShortcuts(shortcuts: Shortcut[]): () => void {
  const handler = (e: KeyboardEvent) => {
    // Guard: don't fire shortcuts when typing in forms
    const target = e.target as HTMLElement;
    if (target?.tagName?.match(/^(INPUT|TEXTAREA|SELECT)$/i)) return;

    for (const shortcut of shortcuts) {
      const metaMatch = shortcut.meta ? (e.metaKey || e.ctrlKey) : true;
      const shiftMatch = shortcut.shift ? e.shiftKey : !e.shiftKey;

      if (e.key === shortcut.key && metaMatch && shiftMatch) {
        e.preventDefault();
        shortcut.action();
        return;
      }
    }
  };

  window.addEventListener('keydown', handler);
  return () => window.removeEventListener('keydown', handler);
}
