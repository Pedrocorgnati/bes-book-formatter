// preferencesStore.ts — preferências do usuário (tema, idioma, analytics)
// Sincronizado com SQLite via IPC Tauri
import { writable } from 'svelte/store';
import type { UserPreferences } from '$lib/types/interfaces';
import { UILanguage } from '$lib/types/enums';

const defaultPreferences: UserPreferences = {
  theme: 'light',
  uiLanguage: UILanguage.PT_BR,
  analyticsOptIn: false
};

export const preferencesStore = writable<UserPreferences>(defaultPreferences);

// Atualiza o data-theme no <html> e localStorage ao mudar o tema
export function applyTheme(theme: 'light' | 'dark'): void {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', theme);
    try {
      localStorage.setItem('bes_theme', theme);
    } catch {
      // localStorage indisponível — sem erro
    }
  }
}

// Inicializa preferências: lê do SQLite via IPC ou usa defaults
export async function initPreferences(): Promise<void> {
  try {
    // TODO: Implementar backend — await ipc('get_preferences')
    // Por ora usa valores do localStorage como cache rápido
    const savedTheme = typeof localStorage !== 'undefined'
      ? localStorage.getItem('bes_theme') as 'light' | 'dark' | null
      : null;
    const savedLang = typeof localStorage !== 'undefined'
      ? localStorage.getItem('bes_language') as UILanguage | null
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

// Muda tema e persiste
export async function setTheme(theme: 'light' | 'dark'): Promise<void> {
  applyTheme(theme);
  preferencesStore.update(p => ({ ...p, theme }));
  try {
    // TODO: Implementar backend — await ipc('set_preference', { key: 'theme', value: theme })
    localStorage.setItem('bes_theme', theme);
  } catch {
    // Persistência indisponível
  }
}

// Muda idioma e persiste
export async function setLanguage(lang: UILanguage): Promise<void> {
  preferencesStore.update(p => ({ ...p, uiLanguage: lang }));
  try {
    // TODO: Implementar backend — await ipc('set_preference', { key: 'language', value: lang })
    localStorage.setItem('bes_language', lang);
  } catch {
    // Persistência indisponível
  }
}

// Alterna analytics opt-in
export async function setAnalyticsOptIn(value: boolean): Promise<void> {
  preferencesStore.update(p => ({ ...p, analyticsOptIn: value }));
  try {
    // TODO: Implementar backend
    localStorage.setItem('bes_analytics', String(value));
  } catch {
    // Persistência indisponível
  }
}
