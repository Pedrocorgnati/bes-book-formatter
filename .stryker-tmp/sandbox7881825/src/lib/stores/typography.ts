// @ts-nocheck
// typographyStore.ts — configuração tipográfica do projeto atual
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
import { writable, derived } from 'svelte/store';
import type { TypographyConfig } from '$lib/types/interfaces';

// Configuração tipográfica do projeto aberto
export const typographyStore = writable<TypographyConfig | null>(null);

// Loading state (enquanto carrega/salva via IPC)
export const typographyLoadingStore = writable<boolean>(stryMutAct_9fa48("472") ? true : (stryCov_9fa48("472"), false));

// Derived: preset atual selecionado
export const currentGenrePreset = derived(typographyStore, stryMutAct_9fa48("473") ? () => undefined : (stryCov_9fa48("473"), $config => stryMutAct_9fa48("474") ? $config?.genrePreset && 'nonfiction' : (stryCov_9fa48("474"), (stryMutAct_9fa48("475") ? $config.genrePreset : (stryCov_9fa48("475"), $config?.genrePreset)) ?? (stryMutAct_9fa48("476") ? "" : (stryCov_9fa48("476"), 'nonfiction')))));

// Derived: página tem formato personalizado?
export const isCustomPageFormat = derived(typographyStore, $config => {
  if (stryMutAct_9fa48("477")) {
    {}
  } else {
    stryCov_9fa48("477");
    if (stryMutAct_9fa48("480") ? false : stryMutAct_9fa48("479") ? true : stryMutAct_9fa48("478") ? $config : (stryCov_9fa48("478", "479", "480"), !$config)) return stryMutAct_9fa48("481") ? true : (stryCov_9fa48("481"), false);
    const stdFormats = stryMutAct_9fa48("482") ? [] : (stryCov_9fa48("482"), [stryMutAct_9fa48("483") ? {} : (stryCov_9fa48("483"), {
      w: 6.0,
      h: 9.0
    }), // 6×9
    stryMutAct_9fa48("484") ? {} : (stryCov_9fa48("484"), {
      w: 5.5,
      h: 8.5
    }), // 5.5×8.5
    stryMutAct_9fa48("485") ? {} : (stryCov_9fa48("485"), {
      w: 7.0,
      h: 10.0
    }), // 7×10
    stryMutAct_9fa48("486") ? {} : (stryCov_9fa48("486"), {
      w: 8.27,
      h: 11.69
    }), // A4
    stryMutAct_9fa48("487") ? {} : (stryCov_9fa48("487"), {
      w: 5.83,
      h: 8.27
    }), // A5
    stryMutAct_9fa48("488") ? {} : (stryCov_9fa48("488"), {
      w: 8.5,
      h: 11.0
    }) // Letter
    ]);
    return stryMutAct_9fa48("489") ? stdFormats.some(f => Math.abs(f.w - $config.pageWidth) < 0.01 && Math.abs(f.h - $config.pageHeight) < 0.01) : (stryCov_9fa48("489"), !(stryMutAct_9fa48("490") ? stdFormats.every(f => Math.abs(f.w - $config.pageWidth) < 0.01 && Math.abs(f.h - $config.pageHeight) < 0.01) : (stryCov_9fa48("490"), stdFormats.some(stryMutAct_9fa48("491") ? () => undefined : (stryCov_9fa48("491"), f => stryMutAct_9fa48("494") ? Math.abs(f.w - $config.pageWidth) < 0.01 || Math.abs(f.h - $config.pageHeight) < 0.01 : stryMutAct_9fa48("493") ? false : stryMutAct_9fa48("492") ? true : (stryCov_9fa48("492", "493", "494"), (stryMutAct_9fa48("497") ? Math.abs(f.w - $config.pageWidth) >= 0.01 : stryMutAct_9fa48("496") ? Math.abs(f.w - $config.pageWidth) <= 0.01 : stryMutAct_9fa48("495") ? true : (stryCov_9fa48("495", "496", "497"), Math.abs(stryMutAct_9fa48("498") ? f.w + $config.pageWidth : (stryCov_9fa48("498"), f.w - $config.pageWidth)) < 0.01)) && (stryMutAct_9fa48("501") ? Math.abs(f.h - $config.pageHeight) >= 0.01 : stryMutAct_9fa48("500") ? Math.abs(f.h - $config.pageHeight) <= 0.01 : stryMutAct_9fa48("499") ? true : (stryCov_9fa48("499", "500", "501"), Math.abs(stryMutAct_9fa48("502") ? f.h + $config.pageHeight : (stryCov_9fa48("502"), f.h - $config.pageHeight)) < 0.01))))))));
  }
});