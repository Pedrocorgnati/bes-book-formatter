// @ts-nocheck
// sidecarStore.ts — disponibilidade dos sidecars (Sharp, Typst, etc.)
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
export interface SidecarAvailability {
  sharp: boolean;
  typst: boolean;
  ghostscript: boolean;
  epubcheck: boolean;
  checkedAt: string | null;
}
const defaultAvailability: SidecarAvailability = stryMutAct_9fa48("421") ? {} : (stryCov_9fa48("421"), {
  sharp: stryMutAct_9fa48("422") ? true : (stryCov_9fa48("422"), false),
  typst: stryMutAct_9fa48("423") ? true : (stryCov_9fa48("423"), false),
  ghostscript: stryMutAct_9fa48("424") ? true : (stryCov_9fa48("424"), false),
  epubcheck: stryMutAct_9fa48("425") ? true : (stryCov_9fa48("425"), false),
  checkedAt: null
});
export const sidecarStore = writable<SidecarAvailability>(defaultAvailability);

// Derived: Sharp disponível para processamento avançado de imagens
export const sharpAvailable = derived(sidecarStore, stryMutAct_9fa48("426") ? () => undefined : (stryCov_9fa48("426"), $s => $s.sharp));

// Derived: geração de PDF disponível
export const pdfGenerationAvailable = derived(sidecarStore, stryMutAct_9fa48("427") ? () => undefined : (stryCov_9fa48("427"), $s => $s.typst));

// Derived: validação EPUB disponível
export const epubValidationAvailable = derived(sidecarStore, stryMutAct_9fa48("428") ? () => undefined : (stryCov_9fa48("428"), $s => $s.epubcheck));