// @ts-nocheck
// generationStore.ts — estado do módulo de geração (module-4)
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
import type { StoredGenerationResult, PreflightResult, GenerationResult } from '$lib/types';
export type GenerationStatus = 'idle' | 'preflight' | 'generating' | 'done' | 'error';
interface GenerationState {
  status: GenerationStatus;
  activeFormat: string | null;
  activePlatform: string | null;
  preflight: PreflightResult | null;
  lastResult: GenerationResult | null;
  history: StoredGenerationResult[];
  historyLoading: boolean;
  error: string | null;
}
function createGenerationStore() {
  if (stryMutAct_9fa48("297")) {
    {}
  } else {
    stryCov_9fa48("297");
    const {
      subscribe,
      set,
      update
    } = writable<GenerationState>(stryMutAct_9fa48("298") ? {} : (stryCov_9fa48("298"), {
      status: stryMutAct_9fa48("299") ? "" : (stryCov_9fa48("299"), 'idle'),
      activeFormat: null,
      activePlatform: null,
      preflight: null,
      lastResult: null,
      history: stryMutAct_9fa48("300") ? ["Stryker was here"] : (stryCov_9fa48("300"), []),
      historyLoading: stryMutAct_9fa48("301") ? true : (stryCov_9fa48("301"), false),
      error: null
    }));
    return stryMutAct_9fa48("302") ? {} : (stryCov_9fa48("302"), {
      subscribe,
      startPreflight(format: string, platform: string) {
        if (stryMutAct_9fa48("303")) {
          {}
        } else {
          stryCov_9fa48("303");
          update(stryMutAct_9fa48("304") ? () => undefined : (stryCov_9fa48("304"), s => stryMutAct_9fa48("305") ? {} : (stryCov_9fa48("305"), {
            ...s,
            status: stryMutAct_9fa48("306") ? "" : (stryCov_9fa48("306"), 'preflight'),
            activeFormat: format,
            activePlatform: platform,
            error: null
          })));
        }
      },
      setPreflight(result: PreflightResult) {
        if (stryMutAct_9fa48("307")) {
          {}
        } else {
          stryCov_9fa48("307");
          update(stryMutAct_9fa48("308") ? () => undefined : (stryCov_9fa48("308"), s => stryMutAct_9fa48("309") ? {} : (stryCov_9fa48("309"), {
            ...s,
            preflight: result
          })));
        }
      },
      startGeneration() {
        if (stryMutAct_9fa48("310")) {
          {}
        } else {
          stryCov_9fa48("310");
          update(stryMutAct_9fa48("311") ? () => undefined : (stryCov_9fa48("311"), s => stryMutAct_9fa48("312") ? {} : (stryCov_9fa48("312"), {
            ...s,
            status: stryMutAct_9fa48("313") ? "" : (stryCov_9fa48("313"), 'generating'),
            error: null
          })));
        }
      },
      setResult(result: GenerationResult) {
        if (stryMutAct_9fa48("314")) {
          {}
        } else {
          stryCov_9fa48("314");
          update(stryMutAct_9fa48("315") ? () => undefined : (stryCov_9fa48("315"), s => stryMutAct_9fa48("316") ? {} : (stryCov_9fa48("316"), {
            ...s,
            status: result.success ? stryMutAct_9fa48("317") ? "" : (stryCov_9fa48("317"), 'done') : stryMutAct_9fa48("318") ? "" : (stryCov_9fa48("318"), 'error'),
            lastResult: result,
            error: result.success ? null : stryMutAct_9fa48("319") ? result.errors[0] && 'Erro desconhecido' : (stryCov_9fa48("319"), result.errors[0] ?? (stryMutAct_9fa48("320") ? "" : (stryCov_9fa48("320"), 'Erro desconhecido')))
          })));
        }
      },
      setError(msg: string) {
        if (stryMutAct_9fa48("321")) {
          {}
        } else {
          stryCov_9fa48("321");
          update(stryMutAct_9fa48("322") ? () => undefined : (stryCov_9fa48("322"), s => stryMutAct_9fa48("323") ? {} : (stryCov_9fa48("323"), {
            ...s,
            status: stryMutAct_9fa48("324") ? "" : (stryCov_9fa48("324"), 'error'),
            error: msg
          })));
        }
      },
      setHistory(items: StoredGenerationResult[]) {
        if (stryMutAct_9fa48("325")) {
          {}
        } else {
          stryCov_9fa48("325");
          update(stryMutAct_9fa48("326") ? () => undefined : (stryCov_9fa48("326"), s => stryMutAct_9fa48("327") ? {} : (stryCov_9fa48("327"), {
            ...s,
            history: items,
            historyLoading: stryMutAct_9fa48("328") ? true : (stryCov_9fa48("328"), false)
          })));
        }
      },
      setHistoryLoading(loading: boolean) {
        if (stryMutAct_9fa48("329")) {
          {}
        } else {
          stryCov_9fa48("329");
          update(stryMutAct_9fa48("330") ? () => undefined : (stryCov_9fa48("330"), s => stryMutAct_9fa48("331") ? {} : (stryCov_9fa48("331"), {
            ...s,
            historyLoading: loading
          })));
        }
      },
      reset() {
        if (stryMutAct_9fa48("332")) {
          {}
        } else {
          stryCov_9fa48("332");
          update(stryMutAct_9fa48("333") ? () => undefined : (stryCov_9fa48("333"), s => stryMutAct_9fa48("334") ? {} : (stryCov_9fa48("334"), {
            ...s,
            status: stryMutAct_9fa48("335") ? "" : (stryCov_9fa48("335"), 'idle'),
            activeFormat: null,
            activePlatform: null,
            preflight: null,
            lastResult: null,
            error: null
          })));
        }
      }
    });
  }
}
export const generationStore = createGenerationStore();