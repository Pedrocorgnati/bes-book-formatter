// @ts-nocheck
// engine.ts — Motor de i18n para BES Book Formatter
// Adaptado do scaffold sveltekit/i18n para esta stack
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
import { writable, get } from 'svelte/store';
import ptBR from './pt-BR.json';
import enUS from './en-US.json';
import esES from './es-ES.json';
export type Locale = 'pt-BR' | 'en-US' | 'es-ES';
type TranslationDict = Record<string, unknown>;
const locales: Record<Locale, TranslationDict> = stryMutAct_9fa48("0") ? {} : (stryCov_9fa48("0"), {
  'pt-BR': (ptBR as TranslationDict),
  'en-US': (enUS as TranslationDict),
  'es-ES': (esES as TranslationDict)
});

// Store reativo do locale atual
export const locale = writable<Locale>(stryMutAct_9fa48("1") ? "" : (stryCov_9fa48("1"), 'pt-BR'));

// Resolve chave com notação de ponto (ex: 'nav.dashboard')
function resolveKey(dict: TranslationDict, key: string): string | undefined {
  if (stryMutAct_9fa48("2")) {
    {}
  } else {
    stryCov_9fa48("2");
    const parts = key.split(stryMutAct_9fa48("3") ? "" : (stryCov_9fa48("3"), '.'));
    let current: unknown = dict;
    for (const part of parts) {
      if (stryMutAct_9fa48("4")) {
        {}
      } else {
        stryCov_9fa48("4");
        if (stryMutAct_9fa48("7") ? typeof current !== 'object' && current === null : stryMutAct_9fa48("6") ? false : stryMutAct_9fa48("5") ? true : (stryCov_9fa48("5", "6", "7"), (stryMutAct_9fa48("9") ? typeof current === 'object' : stryMutAct_9fa48("8") ? false : (stryCov_9fa48("8", "9"), typeof current !== (stryMutAct_9fa48("10") ? "" : (stryCov_9fa48("10"), 'object')))) || (stryMutAct_9fa48("12") ? current !== null : stryMutAct_9fa48("11") ? false : (stryCov_9fa48("11", "12"), current === null)))) return undefined;
        current = (current as Record<string, unknown>)[part];
        if (stryMutAct_9fa48("15") ? current !== undefined : stryMutAct_9fa48("14") ? false : stryMutAct_9fa48("13") ? true : (stryCov_9fa48("13", "14", "15"), current === undefined)) return undefined;
      }
    }
    return (stryMutAct_9fa48("18") ? typeof current !== 'string' : stryMutAct_9fa48("17") ? false : stryMutAct_9fa48("16") ? true : (stryCov_9fa48("16", "17", "18"), typeof current === (stryMutAct_9fa48("19") ? "" : (stryCov_9fa48("19"), 'string')))) ? current : undefined;
  }
}

// Interpolação de valores: {name} → valor
function interpolate(template: string, values?: Record<string, string>): string {
  if (stryMutAct_9fa48("20")) {
    {}
  } else {
    stryCov_9fa48("20");
    if (stryMutAct_9fa48("23") ? false : stryMutAct_9fa48("22") ? true : stryMutAct_9fa48("21") ? values : (stryCov_9fa48("21", "22", "23"), !values)) return template;
    return template.replace(stryMutAct_9fa48("25") ? /\{(\W+)\}/g : stryMutAct_9fa48("24") ? /\{(\w)\}/g : (stryCov_9fa48("24", "25"), /\{(\w+)\}/g), (match, key) => {
      if (stryMutAct_9fa48("26")) {
        {}
      } else {
        stryCov_9fa48("26");
        return (stryMutAct_9fa48("29") ? values[key] === undefined : stryMutAct_9fa48("28") ? false : stryMutAct_9fa48("27") ? true : (stryCov_9fa48("27", "28", "29"), values[key] !== undefined)) ? values[key] : match;
      }
    });
  }
}

// Função de tradução principal
export function t(key: string, values?: Record<string, string>): string {
  if (stryMutAct_9fa48("30")) {
    {}
  } else {
    stryCov_9fa48("30");
    const currentLocale = get(locale);
    const dict = locales[currentLocale];

    // Tenta locale atual
    const found = resolveKey(dict, key);
    if (stryMutAct_9fa48("32") ? false : stryMutAct_9fa48("31") ? true : (stryCov_9fa48("31", "32"), found)) return interpolate(found, values);

    // Fallback: pt-BR
    if (stryMutAct_9fa48("35") ? currentLocale === 'pt-BR' : stryMutAct_9fa48("34") ? false : stryMutAct_9fa48("33") ? true : (stryCov_9fa48("33", "34", "35"), currentLocale !== (stryMutAct_9fa48("36") ? "" : (stryCov_9fa48("36"), 'pt-BR')))) {
      if (stryMutAct_9fa48("37")) {
        {}
      } else {
        stryCov_9fa48("37");
        const fallback = resolveKey(locales[stryMutAct_9fa48("38") ? "" : (stryCov_9fa48("38"), 'pt-BR')], key);
        if (stryMutAct_9fa48("40") ? false : stryMutAct_9fa48("39") ? true : (stryCov_9fa48("39", "40"), fallback)) return interpolate(fallback, values);
      }
    }

    // Chave não encontrada: retorna a chave (para debug)
    if (stryMutAct_9fa48("43") ? typeof console === 'undefined' : stryMutAct_9fa48("42") ? false : stryMutAct_9fa48("41") ? true : (stryCov_9fa48("41", "42", "43"), typeof console !== (stryMutAct_9fa48("44") ? "" : (stryCov_9fa48("44"), 'undefined')))) {
      if (stryMutAct_9fa48("45")) {
        {}
      } else {
        stryCov_9fa48("45");
        console.warn(stryMutAct_9fa48("46") ? `` : (stryCov_9fa48("46"), `[i18n] Chave não encontrada: "${key}" (locale: ${currentLocale})`));
      }
    }
    return key;
  }
}

// Muda o locale ativo
export function setLocale(newLocale: Locale): void {
  if (stryMutAct_9fa48("47")) {
    {}
  } else {
    stryCov_9fa48("47");
    locale.set(newLocale);
    if (stryMutAct_9fa48("50") ? typeof localStorage === 'undefined' : stryMutAct_9fa48("49") ? false : stryMutAct_9fa48("48") ? true : (stryCov_9fa48("48", "49", "50"), typeof localStorage !== (stryMutAct_9fa48("51") ? "" : (stryCov_9fa48("51"), 'undefined')))) {
      if (stryMutAct_9fa48("52")) {
        {}
      } else {
        stryCov_9fa48("52");
        try {
          if (stryMutAct_9fa48("53")) {
            {}
          } else {
            stryCov_9fa48("53");
            localStorage.setItem(stryMutAct_9fa48("54") ? "" : (stryCov_9fa48("54"), 'bes_language'), newLocale);
          }
        } catch {
          // localStorage indisponível
        }
      }
    }
  }
}

// Inicializa o locale (lê do localStorage)
export function initLocale(): void {
  if (stryMutAct_9fa48("55")) {
    {}
  } else {
    stryCov_9fa48("55");
    if (stryMutAct_9fa48("58") ? typeof localStorage === 'undefined' : stryMutAct_9fa48("57") ? false : stryMutAct_9fa48("56") ? true : (stryCov_9fa48("56", "57", "58"), typeof localStorage !== (stryMutAct_9fa48("59") ? "" : (stryCov_9fa48("59"), 'undefined')))) {
      if (stryMutAct_9fa48("60")) {
        {}
      } else {
        stryCov_9fa48("60");
        try {
          if (stryMutAct_9fa48("61")) {
            {}
          } else {
            stryCov_9fa48("61");
            const saved = (localStorage.getItem('bes_language') as Locale | null);
            if (stryMutAct_9fa48("64") ? saved || Object.keys(locales).includes(saved) : stryMutAct_9fa48("63") ? false : stryMutAct_9fa48("62") ? true : (stryCov_9fa48("62", "63", "64"), saved && Object.keys(locales).includes(saved))) {
              if (stryMutAct_9fa48("65")) {
                {}
              } else {
                stryCov_9fa48("65");
                locale.set(saved);
              }
            }
          }
        } catch {
          // localStorage indisponível
        }
      }
    }
  }
}

// Lista de locales disponíveis
export const availableLocales: {
  value: Locale;
  label: string;
}[] = stryMutAct_9fa48("66") ? [] : (stryCov_9fa48("66"), [stryMutAct_9fa48("67") ? {} : (stryCov_9fa48("67"), {
  value: stryMutAct_9fa48("68") ? "" : (stryCov_9fa48("68"), 'pt-BR'),
  label: stryMutAct_9fa48("69") ? "" : (stryCov_9fa48("69"), 'Português (pt-BR)')
}), stryMutAct_9fa48("70") ? {} : (stryCov_9fa48("70"), {
  value: stryMutAct_9fa48("71") ? "" : (stryCov_9fa48("71"), 'en-US'),
  label: stryMutAct_9fa48("72") ? "" : (stryCov_9fa48("72"), 'English (en-US)')
}), stryMutAct_9fa48("73") ? {} : (stryCov_9fa48("73"), {
  value: stryMutAct_9fa48("74") ? "" : (stryCov_9fa48("74"), 'es-ES'),
  label: stryMutAct_9fa48("75") ? "" : (stryCov_9fa48("75"), 'Español (es-ES)')
})]);