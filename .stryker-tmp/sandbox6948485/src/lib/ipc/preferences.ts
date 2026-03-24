// @ts-nocheck
// IPC layer for user preferences — Tauri invoke() calls to Rust backend
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
import { invoke } from '@tauri-apps/api/core';
import type { UserPreferences, ApiResponse } from '$lib/types/interfaces';
export async function ipcGetPreferences(): Promise<UserPreferences | null> {
  if (stryMutAct_9fa48("189")) {
    {}
  } else {
    stryCov_9fa48("189");
    const result = await invoke<ApiResponse<Record<string, string>>>(stryMutAct_9fa48("190") ? "" : (stryCov_9fa48("190"), 'get_preferences'));
    if (stryMutAct_9fa48("193") ? false : stryMutAct_9fa48("192") ? true : stryMutAct_9fa48("191") ? result.data : (stryCov_9fa48("191", "192", "193"), !result.data)) return null;
    return stryMutAct_9fa48("194") ? {} : (stryCov_9fa48("194"), {
      theme: stryMutAct_9fa48("195") ? (result.data['theme'] as 'light' | 'dark') && 'light' : (stryCov_9fa48("195"), (result.data['theme'] as 'light' | 'dark') ?? (stryMutAct_9fa48("196") ? "" : (stryCov_9fa48("196"), 'light'))),
      uiLanguage: ((result.data['ui_language'] ?? 'pt-BR') as UserPreferences['uiLanguage']),
      analyticsOptIn: stryMutAct_9fa48("199") ? result.data['analytics_opt_in'] !== 'true' : stryMutAct_9fa48("198") ? false : stryMutAct_9fa48("197") ? true : (stryCov_9fa48("197", "198", "199"), result.data[stryMutAct_9fa48("200") ? "" : (stryCov_9fa48("200"), 'analytics_opt_in')] === (stryMutAct_9fa48("201") ? "" : (stryCov_9fa48("201"), 'true')))
    });
  }
}
export async function ipcSetPreference<K extends keyof UserPreferences>(key: K, value: UserPreferences[K]): Promise<void> {
  if (stryMutAct_9fa48("202")) {
    {}
  } else {
    stryCov_9fa48("202");
    // Map camelCase keys to snake_case DB keys
    const keyMap: Record<string, string> = stryMutAct_9fa48("203") ? {} : (stryCov_9fa48("203"), {
      theme: stryMutAct_9fa48("204") ? "" : (stryCov_9fa48("204"), 'theme'),
      uiLanguage: stryMutAct_9fa48("205") ? "" : (stryCov_9fa48("205"), 'ui_language'),
      analyticsOptIn: stryMutAct_9fa48("206") ? "" : (stryCov_9fa48("206"), 'analytics_opt_in')
    });
    await invoke(stryMutAct_9fa48("207") ? "" : (stryCov_9fa48("207"), 'set_preference'), stryMutAct_9fa48("208") ? {} : (stryCov_9fa48("208"), {
      key: stryMutAct_9fa48("209") ? keyMap[(key as string)] && key : (stryCov_9fa48("209"), keyMap[(key as string)] ?? key),
      value: String(value)
    }));
  }
}
export async function ipcGetTheme(): Promise<'light' | 'dark' | null> {
  if (stryMutAct_9fa48("210")) {
    {}
  } else {
    stryCov_9fa48("210");
    const prefs = await ipcGetPreferences();
    return stryMutAct_9fa48("211") ? prefs?.theme && null : (stryCov_9fa48("211"), (stryMutAct_9fa48("212") ? prefs.theme : (stryCov_9fa48("212"), prefs?.theme)) ?? null);
  }
}