// @ts-nocheck
// toastStore.ts — sistema de notificações toast
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
export type ToastType = 'success' | 'error' | 'warning' | 'info';
export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration: number; // ms (0 = persistente)
  dismissible: boolean;
}
function createToastStore() {
  if (stryMutAct_9fa48("429")) {
    {}
  } else {
    stryCov_9fa48("429");
    const {
      subscribe,
      update
    } = writable<Toast[]>(stryMutAct_9fa48("430") ? ["Stryker was here"] : (stryCov_9fa48("430"), []));
    let counter = 0;
    function add(toast: Omit<Toast, 'id'>): string {
      if (stryMutAct_9fa48("431")) {
        {}
      } else {
        stryCov_9fa48("431");
        const id = stryMutAct_9fa48("432") ? `` : (stryCov_9fa48("432"), `toast-${stryMutAct_9fa48("433") ? --counter : (stryCov_9fa48("433"), ++counter)}`);
        update(list => {
          if (stryMutAct_9fa48("434")) {
            {}
          } else {
            stryCov_9fa48("434");
            // Máximo 3 toasts visíveis
            const trimmed = (stryMutAct_9fa48("438") ? list.length < 3 : stryMutAct_9fa48("437") ? list.length > 3 : stryMutAct_9fa48("436") ? false : stryMutAct_9fa48("435") ? true : (stryCov_9fa48("435", "436", "437", "438"), list.length >= 3)) ? stryMutAct_9fa48("439") ? list : (stryCov_9fa48("439"), list.slice(1)) : list;
            return stryMutAct_9fa48("440") ? [] : (stryCov_9fa48("440"), [...trimmed, stryMutAct_9fa48("441") ? {} : (stryCov_9fa48("441"), {
              ...toast,
              id
            })]);
          }
        });

        // Auto-dismiss (exceto toasts persistentes)
        if (stryMutAct_9fa48("445") ? toast.duration <= 0 : stryMutAct_9fa48("444") ? toast.duration >= 0 : stryMutAct_9fa48("443") ? false : stryMutAct_9fa48("442") ? true : (stryCov_9fa48("442", "443", "444", "445"), toast.duration > 0)) {
          if (stryMutAct_9fa48("446")) {
            {}
          } else {
            stryCov_9fa48("446");
            setTimeout(stryMutAct_9fa48("447") ? () => undefined : (stryCov_9fa48("447"), () => remove(id)), toast.duration);
          }
        }
        return id;
      }
    }
    function remove(id: string): void {
      if (stryMutAct_9fa48("448")) {
        {}
      } else {
        stryCov_9fa48("448");
        update(stryMutAct_9fa48("449") ? () => undefined : (stryCov_9fa48("449"), list => stryMutAct_9fa48("450") ? list : (stryCov_9fa48("450"), list.filter(stryMutAct_9fa48("451") ? () => undefined : (stryCov_9fa48("451"), t => stryMutAct_9fa48("454") ? t.id === id : stryMutAct_9fa48("453") ? false : stryMutAct_9fa48("452") ? true : (stryCov_9fa48("452", "453", "454"), t.id !== id))))));
      }
    }
    function success(message: string, duration = 4000): string {
      if (stryMutAct_9fa48("455")) {
        {}
      } else {
        stryCov_9fa48("455");
        return add(stryMutAct_9fa48("456") ? {} : (stryCov_9fa48("456"), {
          type: stryMutAct_9fa48("457") ? "" : (stryCov_9fa48("457"), 'success'),
          message,
          duration,
          dismissible: stryMutAct_9fa48("458") ? true : (stryCov_9fa48("458"), false)
        }));
      }
    }
    function error(message: string, duration = 0): string {
      if (stryMutAct_9fa48("459")) {
        {}
      } else {
        stryCov_9fa48("459");
        return add(stryMutAct_9fa48("460") ? {} : (stryCov_9fa48("460"), {
          type: stryMutAct_9fa48("461") ? "" : (stryCov_9fa48("461"), 'error'),
          message,
          duration,
          dismissible: stryMutAct_9fa48("462") ? false : (stryCov_9fa48("462"), true)
        }));
      }
    }
    function warning(message: string, duration = 6000): string {
      if (stryMutAct_9fa48("463")) {
        {}
      } else {
        stryCov_9fa48("463");
        return add(stryMutAct_9fa48("464") ? {} : (stryCov_9fa48("464"), {
          type: stryMutAct_9fa48("465") ? "" : (stryCov_9fa48("465"), 'warning'),
          message,
          duration,
          dismissible: stryMutAct_9fa48("466") ? false : (stryCov_9fa48("466"), true)
        }));
      }
    }
    function info(message: string, duration = 4000): string {
      if (stryMutAct_9fa48("467")) {
        {}
      } else {
        stryCov_9fa48("467");
        return add(stryMutAct_9fa48("468") ? {} : (stryCov_9fa48("468"), {
          type: stryMutAct_9fa48("469") ? "" : (stryCov_9fa48("469"), 'info'),
          message,
          duration,
          dismissible: stryMutAct_9fa48("470") ? true : (stryCov_9fa48("470"), false)
        }));
      }
    }
    return stryMutAct_9fa48("471") ? {} : (stryCov_9fa48("471"), {
      subscribe,
      add,
      remove,
      success,
      error,
      warning,
      info
    });
  }
}
export const toast = createToastStore();
export const toastStore = toast; // Alias para compatibilidade