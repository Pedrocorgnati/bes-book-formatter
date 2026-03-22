// IPC layer for user preferences — Tauri invoke() calls to Rust backend
import { invoke } from '@tauri-apps/api/core';
import type { UserPreferences, ApiResponse } from '$lib/types/interfaces';

export async function ipcGetPreferences(): Promise<UserPreferences | null> {
  const result = await invoke<ApiResponse<Record<string, string>>>('get_preferences');
  if (!result.data) return null;

  return {
    theme: (result.data['theme'] as 'light' | 'dark') ?? 'light',
    uiLanguage: (result.data['ui_language'] ?? 'pt-BR') as UserPreferences['uiLanguage'],
    analyticsOptIn: result.data['analytics_opt_in'] === 'true',
  };
}

export async function ipcSetPreference<K extends keyof UserPreferences>(
  key: K,
  value: UserPreferences[K]
): Promise<void> {
  // Map camelCase keys to snake_case DB keys
  const keyMap: Record<string, string> = {
    theme: 'theme',
    uiLanguage: 'ui_language',
    analyticsOptIn: 'analytics_opt_in',
  };

  await invoke('set_preference', {
    key: keyMap[key as string] ?? key,
    value: String(value),
  });
}

export async function ipcGetTheme(): Promise<'light' | 'dark' | null> {
  const prefs = await ipcGetPreferences();
  return prefs?.theme ?? null;
}
