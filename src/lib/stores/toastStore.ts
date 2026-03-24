// toastStore.ts — sistema de notificações toast
import { writable } from 'svelte/store';
import { TIMING, MAX_VISIBLE_TOASTS } from '$lib/constants/timing';

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
  const timers = new Map<string, ReturnType<typeof setTimeout>>();

  function add(toast: Omit<Toast, 'id'>): string {
    const id = `toast-${++counter}`;
    update(list => {
      const trimmed = list.length >= MAX_VISIBLE_TOASTS ? list.slice(1) : list;
      return [...trimmed, { ...toast, id }];
    });

    // Auto-dismiss (exceto toasts persistentes)
    if (toast.duration > 0) {
      const timer = setTimeout(() => { timers.delete(id); remove(id); }, toast.duration);
      timers.set(id, timer);
    }

    return id;
  }

  function remove(id: string): void {
    const timer = timers.get(id);
    if (timer !== undefined) { clearTimeout(timer); timers.delete(id); }
    update(list => list.filter(t => t.id !== id));
  }

  function success(message: string, duration = TIMING.TOAST_SUCCESS): string {
    return add({ type: 'success', message, duration, dismissible: false });
  }

  function error(message: string, duration = TIMING.TOAST_ERROR_PERSISTENT): string {
    return add({ type: 'error', message, duration, dismissible: true });
  }

  function warning(message: string, duration = TIMING.TOAST_WARNING): string {
    return add({ type: 'warning', message, duration, dismissible: true });
  }

  function info(message: string, duration = TIMING.TOAST_INFO): string {
    return add({ type: 'info', message, duration, dismissible: false });
  }

  return { subscribe, add, remove, success, error, warning, info };
}

export const toast = createToastStore();
export const toastStore = toast; // Alias para compatibilidade
