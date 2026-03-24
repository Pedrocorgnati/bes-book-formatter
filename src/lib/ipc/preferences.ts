// IPC layer for user preferences — Tauri invoke() calls to Rust backend
import { ipc } from '$lib/utils/ipc';
import type { UserPreferences, ApiResponse } from '$lib/types/interfaces';

/** Mapeamento das chaves camelCase do front para as chaves snake_case do banco */
const PREF_DB_KEYS = {
  theme:          'theme',
  uiLanguage:     'ui_language',
  analyticsOptIn: 'analytics_opt_in',
} as const satisfies Record<keyof UserPreferences, string>;

/** Valor padrão de idioma de UI quando não há preferência salva */
const DEFAULT_UI_LANGUAGE = 'pt-BR';

/** Valor padrão de tema quando não há preferência salva */
const DEFAULT_THEME = 'light' as const;

export async function ipcGetPreferences(): Promise<UserPreferences | null> {
  const result = await ipc<ApiResponse<Record<string, string>>>('get_preferences');
  if (!result.data) return null;

  return {
    theme: (result.data[PREF_DB_KEYS.theme] as 'light' | 'dark') ?? DEFAULT_THEME,
    uiLanguage: (result.data[PREF_DB_KEYS.uiLanguage] ?? DEFAULT_UI_LANGUAGE) as UserPreferences['uiLanguage'],
    analyticsOptIn: result.data[PREF_DB_KEYS.analyticsOptIn] === 'true',
  };
}

export async function ipcSetPreference<K extends keyof UserPreferences>(
  key: K,
  value: UserPreferences[K]
): Promise<void> {
  await ipc<ApiResponse<null>>('set_preference', {
    key: PREF_DB_KEYS[key] ?? key,
    value: String(value),
  });
}

export async function ipcGetTheme(): Promise<'light' | 'dark' | null> {
  const prefs = await ipcGetPreferences();
  return prefs?.theme ?? null;
}
