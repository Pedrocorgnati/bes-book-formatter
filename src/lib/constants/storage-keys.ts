// ==========================================================
// BES Book Formatter — Storage Keys Centralizados
// Uso: localStorage (cache de arranque) + SQLite (fonte de verdade)
// ==========================================================

export const STORAGE_KEYS = {
  /** Tema da UI ('light' | 'dark') */
  THEME: 'bes_theme',
  /** Locale da UI (UILanguage) */
  LANGUAGE: 'bes_language',
  /** Opt-in de analytics (boolean como string) */
  ANALYTICS: 'bes_analytics',
  /** Flag de primeiro lançamento — grava 'done' após onboarding inicial */
  FIRST_LAUNCH: 'bes_first_launch',
} as const;

export type StorageKey = typeof STORAGE_KEYS[keyof typeof STORAGE_KEYS];
