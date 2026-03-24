// engine.ts — Motor de i18n para BES Book Formatter
// Adaptado do scaffold sveltekit/i18n para esta stack
import { writable, get } from 'svelte/store';
import { STORAGE_KEYS } from '$lib/constants/storage-keys';
import ptBR from './pt-BR.json';
import enUS from './en-US.json';
import esES from './es-ES.json';

export type Locale = 'pt-BR' | 'en-US' | 'es-ES';

type TranslationDict = Record<string, unknown>;

const locales: Record<Locale, TranslationDict> = {
  'pt-BR': ptBR as TranslationDict,
  'en-US': enUS as TranslationDict,
  'es-ES': esES as TranslationDict
};

// Store reativo do locale atual
export const locale = writable<Locale>('pt-BR');

// Resolve chave com notação de ponto (ex: 'nav.dashboard')
function resolveKey(dict: TranslationDict, key: string): string | undefined {
  const parts = key.split('.');
  let current: unknown = dict;
  for (const part of parts) {
    if (typeof current !== 'object' || current === null) return undefined;
    current = (current as Record<string, unknown>)[part];
    if (current === undefined) return undefined;
  }
  return typeof current === 'string' ? current : undefined;
}

// Interpolação de valores: {name} → valor
function interpolate(template: string, values?: Record<string, string>): string {
  if (!values) return template;
  return template.replace(/\{(\w+)\}/g, (match, key) => {
    return values[key] !== undefined ? values[key] : match;
  });
}

// Função de tradução principal
export function t(key: string, values?: Record<string, string>): string {
  const currentLocale = get(locale);
  const dict = locales[currentLocale];

  // Tenta locale atual
  const found = resolveKey(dict, key);
  if (found) return interpolate(found, values);

  // Fallback: pt-BR
  if (currentLocale !== 'pt-BR') {
    const fallback = resolveKey(locales['pt-BR'], key);
    if (fallback) return interpolate(fallback, values);
  }

  // Chave não encontrada: retorna a chave (para debug)
  if (typeof console !== 'undefined') {
    console.warn(`[i18n] Chave não encontrada: "${key}" (locale: ${currentLocale})`);
  }
  return key;
}

// Muda o locale ativo
export function setLocale(newLocale: Locale): void {
  locale.set(newLocale);
  if (typeof localStorage !== 'undefined') {
    try {
      localStorage.setItem(STORAGE_KEYS.LANGUAGE, newLocale);
    } catch {
      // localStorage indisponível
    }
  }
}

// Inicializa o locale (lê do localStorage)
export function initLocale(): void {
  if (typeof localStorage !== 'undefined') {
    try {
      const saved = localStorage.getItem(STORAGE_KEYS.LANGUAGE) as Locale | null;
      if (saved && Object.keys(locales).includes(saved)) {
        locale.set(saved);
      }
    } catch {
      // localStorage indisponível
    }
  }
}

// Lista de locales disponíveis
export const availableLocales: { value: Locale; label: string }[] = [
  { value: 'pt-BR', label: 'Português (pt-BR)' },
  { value: 'en-US', label: 'English (en-US)' },
  { value: 'es-ES', label: 'Español (es-ES)' }
];

