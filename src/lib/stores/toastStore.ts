// toastStore.ts — sistema de notificações toast
import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration: number; // ms (0 = persistente)
  dismissible: boolean;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);

  let counter = 0;

  function add(toast: Omit<Toast, 'id'>): string {
    const id = `toast-${++counter}`;
    update(list => {
      // Máximo 3 toasts visíveis
      const trimmed = list.length >= 3 ? list.slice(1) : list;
      return [...trimmed, { ...toast, id }];
    });

    // Auto-dismiss (exceto toasts persistentes)
    if (toast.duration > 0) {
      setTimeout(() => remove(id), toast.duration);
    }

    return id;
  }

  function remove(id: string): void {
    update(list => list.filter(t => t.id !== id));
  }

  function success(message: string, duration = 4000): string {
    return add({ type: 'success', message, duration, dismissible: false });
  }

  function error(message: string, duration = 0): string {
    return add({ type: 'error', message, duration, dismissible: true });
  }

  function warning(message: string, duration = 6000): string {
    return add({ type: 'warning', message, duration, dismissible: true });
  }

  function info(message: string, duration = 4000): string {
    return add({ type: 'info', message, duration, dismissible: false });
  }

  return { subscribe, add, remove, success, error, warning, info };
}

export const toast = createToastStore();
