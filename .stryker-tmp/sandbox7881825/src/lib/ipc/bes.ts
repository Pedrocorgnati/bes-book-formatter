// @ts-nocheck
// IPC layer for BES integration — Tauri invoke() calls to Rust backend
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
import type { BesWorkspaceInfo, BesDocuments, BesMetadata, EditorialProgress } from '$lib/types/bes';
import type { ApiResponse } from '$lib/types';
export async function ipcValidateBesWorkspace(workspacePath: string): Promise<BesWorkspaceInfo> {
  if (stryMutAct_9fa48("76")) {
    {}
  } else {
    stryCov_9fa48("76");
    const result = await ipc<ApiResponse<BesWorkspaceInfo>>(stryMutAct_9fa48("77") ? "" : (stryCov_9fa48("77"), 'validate_bes_workspace'), stryMutAct_9fa48("78") ? {} : (stryCov_9fa48("78"), {
      workspacePath
    }));
    if (stryMutAct_9fa48("80") ? false : stryMutAct_9fa48("79") ? true : (stryCov_9fa48("79", "80"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcReadBesDocs(projectId: string, workspacePath: string): Promise<BesDocuments> {
  if (stryMutAct_9fa48("81")) {
    {}
  } else {
    stryCov_9fa48("81");
    const result = await ipc<ApiResponse<BesDocuments>>(stryMutAct_9fa48("82") ? "" : (stryCov_9fa48("82"), 'read_bes_docs'), stryMutAct_9fa48("83") ? {} : (stryCov_9fa48("83"), {
      projectId,
      workspacePath
    }));
    if (stryMutAct_9fa48("85") ? false : stryMutAct_9fa48("84") ? true : (stryCov_9fa48("84", "85"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGetBesMetadata(projectId: string, workspacePath: string): Promise<BesMetadata | null> {
  if (stryMutAct_9fa48("86")) {
    {}
  } else {
    stryCov_9fa48("86");
    const result = await ipc<ApiResponse<BesMetadata>>(stryMutAct_9fa48("87") ? "" : (stryCov_9fa48("87"), 'get_bes_metadata'), stryMutAct_9fa48("88") ? {} : (stryCov_9fa48("88"), {
      projectId,
      workspacePath
    }));
    if (stryMutAct_9fa48("90") ? false : stryMutAct_9fa48("89") ? true : (stryCov_9fa48("89", "90"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("91") ? result.data && null : (stryCov_9fa48("91"), result.data ?? null);
  }
}
export async function ipcInvalidateBesCache(projectId: string): Promise<void> {
  if (stryMutAct_9fa48("92")) {
    {}
  } else {
    stryCov_9fa48("92");
    const result = await ipc<ApiResponse<boolean>>(stryMutAct_9fa48("93") ? "" : (stryCov_9fa48("93"), 'invalidate_bes_cache'), stryMutAct_9fa48("94") ? {} : (stryCov_9fa48("94"), {
      projectId
    }));
    if (stryMutAct_9fa48("96") ? false : stryMutAct_9fa48("95") ? true : (stryCov_9fa48("95", "96"), result.error)) throw new Error(result.error);
  }
}
export async function ipcSyncEditorialProgress(projectId: string, workspacePath: string, projectName: string): Promise<EditorialProgress> {
  if (stryMutAct_9fa48("97")) {
    {}
  } else {
    stryCov_9fa48("97");
    const result = await ipc<ApiResponse<EditorialProgress>>(stryMutAct_9fa48("98") ? "" : (stryCov_9fa48("98"), 'sync_editorial_progress'), stryMutAct_9fa48("99") ? {} : (stryCov_9fa48("99"), {
      projectId,
      workspacePath,
      projectName
    }));
    if (stryMutAct_9fa48("101") ? false : stryMutAct_9fa48("100") ? true : (stryCov_9fa48("100", "101"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcUpdateEditorialF10(projectId: string, workspacePath: string, projectName: string, formatsGenerated: string[], outputPath: string): Promise<EditorialProgress> {
  if (stryMutAct_9fa48("102")) {
    {}
  } else {
    stryCov_9fa48("102");
    const result = await ipc<ApiResponse<EditorialProgress>>(stryMutAct_9fa48("103") ? "" : (stryCov_9fa48("103"), 'update_editorial_f10'), stryMutAct_9fa48("104") ? {} : (stryCov_9fa48("104"), {
      projectId,
      workspacePath,
      projectName,
      formatsGenerated,
      outputPath
    }));
    if (stryMutAct_9fa48("106") ? false : stryMutAct_9fa48("105") ? true : (stryCov_9fa48("105", "106"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}