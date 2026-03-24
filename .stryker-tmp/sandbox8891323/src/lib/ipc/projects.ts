// @ts-nocheck
// IPC layer for projects — Tauri invoke() calls to Rust backend
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
import { ipc } from '$lib/utils/ipc';
import type { BookProject, ApiResponse } from '$lib/types/interfaces';
export async function ipcGetProjects(limit: number = 20): Promise<BookProject[]> {
  if (stryMutAct_9fa48("213")) {
    {}
  } else {
    stryCov_9fa48("213");
    const result = await ipc<ApiResponse<BookProject[]>>(stryMutAct_9fa48("214") ? "" : (stryCov_9fa48("214"), 'get_projects'), stryMutAct_9fa48("215") ? {} : (stryCov_9fa48("215"), {
      limit
    }));
    return stryMutAct_9fa48("216") ? result.data && [] : (stryCov_9fa48("216"), result.data ?? (stryMutAct_9fa48("217") ? ["Stryker was here"] : (stryCov_9fa48("217"), [])));
  }
}
export async function ipcGetProject(id: string): Promise<BookProject | null> {
  if (stryMutAct_9fa48("218")) {
    {}
  } else {
    stryCov_9fa48("218");
    const result = await ipc<ApiResponse<BookProject>>(stryMutAct_9fa48("219") ? "" : (stryCov_9fa48("219"), 'get_project'), stryMutAct_9fa48("220") ? {} : (stryCov_9fa48("220"), {
      id
    }));
    return stryMutAct_9fa48("221") ? result.data && null : (stryCov_9fa48("221"), result.data ?? null);
  }
}
export async function ipcImportProject(besRoot: string): Promise<BookProject | null> {
  if (stryMutAct_9fa48("222")) {
    {}
  } else {
    stryCov_9fa48("222");
    const result = await ipc<ApiResponse<BookProject>>(stryMutAct_9fa48("223") ? "" : (stryCov_9fa48("223"), 'import_project'), stryMutAct_9fa48("224") ? {} : (stryCov_9fa48("224"), {
      besRoot
    }));
    if (stryMutAct_9fa48("226") ? false : stryMutAct_9fa48("225") ? true : (stryCov_9fa48("225", "226"), result.error)) {
      if (stryMutAct_9fa48("227")) {
        {}
      } else {
        stryCov_9fa48("227");
        throw new Error(result.error);
      }
    }
    return stryMutAct_9fa48("228") ? result.data && null : (stryCov_9fa48("228"), result.data ?? null);
  }
}
export async function ipcDeleteProject(id: string): Promise<void> {
  if (stryMutAct_9fa48("229")) {
    {}
  } else {
    stryCov_9fa48("229");
    const result = await ipc<ApiResponse<boolean>>(stryMutAct_9fa48("230") ? "" : (stryCov_9fa48("230"), 'delete_project'), stryMutAct_9fa48("231") ? {} : (stryCov_9fa48("231"), {
      id
    }));
    if (stryMutAct_9fa48("233") ? false : stryMutAct_9fa48("232") ? true : (stryCov_9fa48("232", "233"), result.error)) {
      if (stryMutAct_9fa48("234")) {
        {}
      } else {
        stryCov_9fa48("234");
        throw new Error(result.error);
      }
    }
  }
}
export async function ipcInitDatabase(): Promise<void> {
  if (stryMutAct_9fa48("235")) {
    {}
  } else {
    stryCov_9fa48("235");
    await ipc<void>(stryMutAct_9fa48("236") ? "" : (stryCov_9fa48("236"), 'init_database'));
  }
}