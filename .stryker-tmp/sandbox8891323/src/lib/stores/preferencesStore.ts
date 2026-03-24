// @ts-nocheck
// preferencesStore.ts — preferências do usuário (tema, idioma, analytics)
// Sincronizado com SQLite via IPC Tauri
function stryNS_9fa48() {
  var g = typeof globalThis === 'object' && globalThis && globalThis.Math === Math && globalThis || new Function("return this")();
  var ns = g.__stryker__ || (g.__stryker__ = {});
  if (ns.activeMutant === undefined && g.process && g.process.env && g.process.env.__STRYKER_ACTIVE_MUTANT__) {
    ns.activeMutant = g.process.env.__STRYKER_ACTIVE_MUTANT__;
  }
  function retrieveNS() {
    return ns;
  }
  stryNS_9fa48 = retrieveNS;
  return retrieveNS();
}
stryNS_9fa48();
function stryCov_9fa48() {
  var ns = stryNS_9fa48();
  var cov = ns.mutantCoverage || (ns.mutantCoverage = {
    static: {},
    perTest: {}
  });
  function cover() {
    var c = cov.static;
    if (ns.currentTestId) {
      c = cov.perTest[ns.currentTestId] = cov.perTest[ns.currentTestId] || {};
    }
    var a = arguments;
    for (var i = 0; i < a.length; i++) {
      c[a[i]] = (c[a[i]] || 0) + 1;
    }
  }
  stryCov_9fa48 = cover;
  cover.apply(null, arguments);
}
function stryMutAct_9fa48(id) {
  var ns = stryNS_9fa48();
  function isActive(id) {
    if (ns.activeMutant === id) {
      if (ns.hitCount !== void 0 && ++ns.hitCount > ns.hitLimit) {
        throw new Error('Stryker: Hit count limit reached (' + ns.hitCount + ')');
      }
      return true;
    }
    return false;
  }
  stryMutAct_9fa48 = isActive;
  return isActive(id);
}
import { writable } from 'svelte/store';
import type { UserPreferences } from '$lib/types/interfaces';
import { UILanguage } from '$lib/types/enums';
const defaultPreferences: UserPreferences = stryMutAct_9fa48("338") ? {} : (stryCov_9fa48("338"), {
  theme: stryMutAct_9fa48("339") ? "" : (stryCov_9fa48("339"), 'light'),
  uiLanguage: UILanguage.PT_BR,
  analyticsOptIn: stryMutAct_9fa48("340") ? true : (stryCov_9fa48("340"), false)
});
export const preferencesStore = writable<UserPreferences>(defaultPreferences);

// Atualiza o data-theme no <html> e localStorage ao mudar o tema
export function applyTheme(theme: 'light' | 'dark'): void {
  if (stryMutAct_9fa48("341")) {
    {}
  } else {
    stryCov_9fa48("341");
    if (stryMutAct_9fa48("344") ? typeof document === 'undefined' : stryMutAct_9fa48("343") ? false : stryMutAct_9fa48("342") ? true : (stryCov_9fa48("342", "343", "344"), typeof document !== (stryMutAct_9fa48("345") ? "" : (stryCov_9fa48("345"), 'undefined')))) {
      if (stryMutAct_9fa48("346")) {
        {}
      } else {
        stryCov_9fa48("346");
        document.documentElement.setAttribute(stryMutAct_9fa48("347") ? "" : (stryCov_9fa48("347"), 'data-theme'), theme);
        try {
          if (stryMutAct_9fa48("348")) {
            {}
          } else {
            stryCov_9fa48("348");
            localStorage.setItem(stryMutAct_9fa48("349") ? "" : (stryCov_9fa48("349"), 'bes_theme'), theme);
          }
        } catch {
          // localStorage indisponível — sem erro
        }
      }
    }
  }
}

// Inicializa preferências: lê do SQLite via IPC ou usa defaults
export async function initPreferences(): Promise<void> {
  if (stryMutAct_9fa48("350")) {
    {}
  } else {
    stryCov_9fa48("350");
    try {
      if (stryMutAct_9fa48("351")) {
        {}
      } else {
        stryCov_9fa48("351");
        // TODO: Implementar backend — await ipc('get_preferences')
        // Por ora usa valores do localStorage como cache rápido
        const savedTheme = (stryMutAct_9fa48("354") ? typeof localStorage === 'undefined' : stryMutAct_9fa48("353") ? false : stryMutAct_9fa48("352") ? true : (stryCov_9fa48("352", "353", "354"), typeof localStorage !== (stryMutAct_9fa48("355") ? "" : (stryCov_9fa48("355"), 'undefined')))) ? (localStorage.getItem('bes_theme') as 'light' | 'dark' | null) : null;
        const savedLang = (stryMutAct_9fa48("358") ? typeof localStorage === 'undefined' : stryMutAct_9fa48("357") ? false : stryMutAct_9fa48("356") ? true : (stryCov_9fa48("356", "357", "358"), typeof localStorage !== (stryMutAct_9fa48("359") ? "" : (stryCov_9fa48("359"), 'undefined')))) ? (localStorage.getItem('bes_language') as UILanguage | null) : null;
        preferencesStore.update(stryMutAct_9fa48("360") ? () => undefined : (stryCov_9fa48("360"), p => stryMutAct_9fa48("361") ? {} : (stryCov_9fa48("361"), {
          ...p,
          theme: stryMutAct_9fa48("362") ? savedTheme && p.theme : (stryCov_9fa48("362"), savedTheme ?? p.theme),
          uiLanguage: stryMutAct_9fa48("363") ? savedLang && p.uiLanguage : (stryCov_9fa48("363"), savedLang ?? p.uiLanguage)
        })));
        applyTheme(stryMutAct_9fa48("364") ? savedTheme && defaultPreferences.theme : (stryCov_9fa48("364"), savedTheme ?? defaultPreferences.theme));
      }
    } catch {
      if (stryMutAct_9fa48("365")) {
        {}
      } else {
        stryCov_9fa48("365");
        applyTheme(defaultPreferences.theme);
      }
    }
  }
}

// Muda tema e persiste
export async function setTheme(theme: 'light' | 'dark'): Promise<void> {
  if (stryMutAct_9fa48("366")) {
    {}
  } else {
    stryCov_9fa48("366");
    applyTheme(theme);
    preferencesStore.update(stryMutAct_9fa48("367") ? () => undefined : (stryCov_9fa48("367"), p => stryMutAct_9fa48("368") ? {} : (stryCov_9fa48("368"), {
      ...p,
      theme
    })));
    try {
      if (stryMutAct_9fa48("369")) {
        {}
      } else {
        stryCov_9fa48("369");
        // TODO: Implementar backend — await ipc('set_preference', { key: 'theme', value: theme })
        localStorage.setItem(stryMutAct_9fa48("370") ? "" : (stryCov_9fa48("370"), 'bes_theme'), theme);
      }
    } catch {
      // Persistência indisponível
    }
  }
}

// Muda idioma e persiste
export async function setLanguage(lang: UILanguage): Promise<void> {
  if (stryMutAct_9fa48("371")) {
    {}
  } else {
    stryCov_9fa48("371");
    preferencesStore.update(stryMutAct_9fa48("372") ? () => undefined : (stryCov_9fa48("372"), p => stryMutAct_9fa48("373") ? {} : (stryCov_9fa48("373"), {
      ...p,
      uiLanguage: lang
    })));
    try {
      if (stryMutAct_9fa48("374")) {
        {}
      } else {
        stryCov_9fa48("374");
        // TODO: Implementar backend — await ipc('set_preference', { key: 'language', value: lang })
        localStorage.setItem(stryMutAct_9fa48("375") ? "" : (stryCov_9fa48("375"), 'bes_language'), lang);
      }
    } catch {
      // Persistência indisponível
    }
  }
}

// Alterna analytics opt-in
export async function setAnalyticsOptIn(value: boolean): Promise<void> {
  if (stryMutAct_9fa48("376")) {
    {}
  } else {
    stryCov_9fa48("376");
    preferencesStore.update(stryMutAct_9fa48("377") ? () => undefined : (stryCov_9fa48("377"), p => stryMutAct_9fa48("378") ? {} : (stryCov_9fa48("378"), {
      ...p,
      analyticsOptIn: value
    })));
    try {
      if (stryMutAct_9fa48("379")) {
        {}
      } else {
        stryCov_9fa48("379");
        // TODO: Implementar backend
        localStorage.setItem(stryMutAct_9fa48("380") ? "" : (stryCov_9fa48("380"), 'bes_analytics'), String(value));
      }
    } catch {
      // Persistência indisponível
    }
  }
}