/**
 * WCAG AA accessibility helpers
 */
import { TIMING } from '$lib/constants/timing';

/**
 * Creates a focus trap within a container element.
 * Returns a cleanup function.
 */
export function trapFocus(container: HTMLElement): () => void {
  const focusableSelector = 'a[href], button:not([disabled]), textarea, input, select, [tabindex]:not([tabindex="-1"])';

  const handler = (e: KeyboardEvent) => {
    if (e.key !== 'Tab') return;

    const focusable = Array.from(container.querySelectorAll<HTMLElement>(focusableSelector));
    if (focusable.length === 0) return;

    const first: HTMLElement | undefined = focusable[0];
    const last: HTMLElement | undefined = focusable[focusable.length - 1];
    if (!first || !last) return;

    if (e.shiftKey) {
      if (document.activeElement === first) {
        e.preventDefault();
        last.focus();
      }
    } else {
      if (document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  };

  container.addEventListener('keydown', handler);
  return () => container.removeEventListener('keydown', handler);
}

/**
 * Announces a message to screen readers via a live region.
 */
export function announceToScreenReader(
  message: string,
  priority: 'polite' | 'assertive' = 'polite'
): void {
  const el = document.createElement('div');
  el.setAttribute('aria-live', priority);
  el.setAttribute('aria-atomic', 'true');
  el.setAttribute('class', 'sr-only');
  el.style.cssText = 'position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0);';
  document.body.appendChild(el);

  // Delay to ensure screen reader picks up the change
  requestAnimationFrame(() => {
    el.textContent = message;
    setTimeout(() => el.remove(), TIMING.A11Y_LIVE_REGION_REMOVE);
  });
}

/**
 * Checks if user prefers reduced motion.
 */
export function prefersReducedMotion(): boolean {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}
