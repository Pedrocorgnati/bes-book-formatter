/**
 * Centralized keyboard shortcuts handler.
 * Ignores events when focus is in INPUT/TEXTAREA/SELECT.
 */
// @ts-nocheck
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
interface Shortcut {
  key: string;
  meta?: boolean; // Cmd (Mac) / Ctrl (Windows/Linux)
  shift?: boolean;
  action: () => void;
}
export function registerShortcuts(shortcuts: Shortcut[]): () => void {
  if (stryMutAct_9fa48("570")) {
    {}
  } else {
    stryCov_9fa48("570");
    const handler = (e: KeyboardEvent) => {
      if (stryMutAct_9fa48("571")) {
        {}
      } else {
        stryCov_9fa48("571");
        // Guard: don't fire shortcuts when typing in forms
        const target = (e.target as HTMLElement);
        if (stryMutAct_9fa48("575") ? target.tagName?.match(/^(INPUT|TEXTAREA|SELECT)$/i) : stryMutAct_9fa48("574") ? target?.tagName.match(/^(INPUT|TEXTAREA|SELECT)$/i) : stryMutAct_9fa48("573") ? false : stryMutAct_9fa48("572") ? true : (stryCov_9fa48("572", "573", "574", "575"), target?.tagName?.match(stryMutAct_9fa48("577") ? /^(INPUT|TEXTAREA|SELECT)/i : stryMutAct_9fa48("576") ? /(INPUT|TEXTAREA|SELECT)$/i : (stryCov_9fa48("576", "577"), /^(INPUT|TEXTAREA|SELECT)$/i)))) return;
        for (const shortcut of shortcuts) {
          if (stryMutAct_9fa48("578")) {
            {}
          } else {
            stryCov_9fa48("578");
            const metaMatch = shortcut.meta ? stryMutAct_9fa48("581") ? e.metaKey && e.ctrlKey : stryMutAct_9fa48("580") ? false : stryMutAct_9fa48("579") ? true : (stryCov_9fa48("579", "580", "581"), e.metaKey || e.ctrlKey) : stryMutAct_9fa48("582") ? false : (stryCov_9fa48("582"), true);
            const shiftMatch = shortcut.shift ? e.shiftKey : stryMutAct_9fa48("583") ? e.shiftKey : (stryCov_9fa48("583"), !e.shiftKey);
            if (stryMutAct_9fa48("586") ? e.key === shortcut.key && metaMatch || shiftMatch : stryMutAct_9fa48("585") ? false : stryMutAct_9fa48("584") ? true : (stryCov_9fa48("584", "585", "586"), (stryMutAct_9fa48("588") ? e.key === shortcut.key || metaMatch : stryMutAct_9fa48("587") ? true : (stryCov_9fa48("587", "588"), (stryMutAct_9fa48("590") ? e.key !== shortcut.key : stryMutAct_9fa48("589") ? true : (stryCov_9fa48("589", "590"), e.key === shortcut.key)) && metaMatch)) && shiftMatch)) {
              if (stryMutAct_9fa48("591")) {
                {}
              } else {
                stryCov_9fa48("591");
                e.preventDefault();
                shortcut.action();
                return;
              }
            }
          }
        }
      }
    };
    window.addEventListener(stryMutAct_9fa48("592") ? "" : (stryCov_9fa48("592"), 'keydown'), handler);
    return stryMutAct_9fa48("593") ? () => undefined : (stryCov_9fa48("593"), () => window.removeEventListener(stryMutAct_9fa48("594") ? "" : (stryCov_9fa48("594"), 'keydown'), handler));
  }
}