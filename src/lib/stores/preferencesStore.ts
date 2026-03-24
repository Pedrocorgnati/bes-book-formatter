// preferencesStore.ts — preferências do usuário (tema, idioma, analytics)
// Sincronizado com SQLite via IPC Tauri, com localStorage como cache de arranque
import { writable } from 'svelte/store';
import type { UserPreferences } from '$lib/types/interfaces';
import { UILanguage } from '$lib/types/enums';
import { ipcGetPreferences, ipcSetPreference } from '$lib/ipc/preferences';
import { STORAGE_KEYS } from '$lib/constants/storage-keys';

const defaultPreferences: UserPreferences = {
  theme: 'light',
  uiLanguage: UILanguage.PT_BR,
  analyticsOptIn: false
};

export const preferencesStore = writable<UserPreferences>(defaultPreferences);

// Atualiza o data-theme no <html> ao mudar o tema
export function applyTheme(theme: 'light' | 'dark'): void {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', theme);
  }
}

// Inicializa preferências: carrega do SQLite via IPC, com fallback para localStorage
export async function initPreferences(): Promise<void> {
  try {
    const prefs = await ipcGetPreferences();
    if (prefs) {
      preferencesStore.set(prefs);
      applyTheme(prefs.theme);
      return;
    }
  } catch {
    // Backend indisponível na inicialização — usa cache local
  }

  // Fallback: lê do localStorage para arranque rápido
  try {
    const savedTheme = typeof localStorage !== 'undefined'
      ? localStorage.getItem(STORAGE_KEYS.THEME) as 'light' | 'dark' | null
      : null;
    const savedLang = typeof localStorage !== 'undefined'
      ? localStorage.getItem(STORAGE_KEYS.LANGUAGE) as UILanguage | null
      : null;

    preferencesStore.update(p => ({
      ...p,
      theme: savedTheme ?? p.theme,
      uiLanguage: savedLang ?? p.uiLanguage
    }));

    applyTheme(savedTheme ?? defaultPreferences.theme);
  } catch {
    applyTheme(defaultPreferences.theme);
  }
}

// Muda tema e persiste no backend + localStorage
export async function setTheme(theme: 'light' | 'dark'): Promise<void> {
  applyTheme(theme);
  preferencesStore.update(p => ({ ...p, theme }));
  try {
    await ipcSetPreference('theme', theme);
    localStorage.setItem(STORAGE_KEYS.THEME, theme);
  } catch {
    // Persiste só no localStorage se o backend falhar
    try { localStorage.setItem(STORAGE_KEYS.THEME, theme); } catch { /* noop */ }
  }
}

// Muda idioma e persiste no backend + localStorage
export async function setLanguage(lang: UILanguage): Promise<void> {
  preferencesStore.update(p => ({ ...p, uiLanguage: lang }));
  try {
    await ipcSetPreference('uiLanguage', lang);
    localStorage.setItem(STORAGE_KEYS.LANGUAGE, lang);
  } catch {
    try { localStorage.setItem(STORAGE_KEYS.LANGUAGE, lang); } catch { /* noop */ }
  }
}

// Alterna analytics opt-in e persiste
export async function setAnalyticsOptIn(value: boolean): Promise<void> {
  preferencesStore.update(p => ({ ...p, analyticsOptIn: value }));
  try {
    await ipcSetPreference('analyticsOptIn', value);
    localStorage.setItem(STORAGE_KEYS.ANALYTICS, String(value));
  } catch {
    try { localStorage.setItem(STORAGE_KEYS.ANALYTICS, String(value)); } catch { /* noop */ }
  }
}
