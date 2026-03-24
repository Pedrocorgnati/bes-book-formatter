// @ts-nocheck
// IPC layer for cover design (module-7) — Tauri invoke() calls
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
import type { ApiResponse, CoverConfig, CoverConfigInput, CoverTemplate, SpineWidthResult } from '$lib/types/interfaces';
export async function ipcGetCoverConfig(projectId: string): Promise<CoverConfig | null> {
  if (stryMutAct_9fa48("107")) {
    {}
  } else {
    stryCov_9fa48("107");
    const resp = await ipc<ApiResponse<CoverConfig | null>>(stryMutAct_9fa48("108") ? "" : (stryCov_9fa48("108"), 'get_cover_config'), stryMutAct_9fa48("109") ? {} : (stryCov_9fa48("109"), {
      projectId
    }));
    if (stryMutAct_9fa48("111") ? false : stryMutAct_9fa48("110") ? true : (stryCov_9fa48("110", "111"), resp.error)) throw new Error(resp.error);
    return stryMutAct_9fa48("112") ? resp.data && null : (stryCov_9fa48("112"), resp.data ?? null);
  }
}
export async function ipcCalculateSpineWidth(projectId: string, platform: string, paperType: string): Promise<{
  result: SpineWidthResult;
  warnings: string[];
}> {
  if (stryMutAct_9fa48("113")) {
    {}
  } else {
    stryCov_9fa48("113");
    const resp = await ipc<ApiResponse<SpineWidthResult>>(stryMutAct_9fa48("114") ? "" : (stryCov_9fa48("114"), 'calculate_spine_width'), stryMutAct_9fa48("115") ? {} : (stryCov_9fa48("115"), {
      projectId,
      platform,
      paperType
    }));
    if (stryMutAct_9fa48("117") ? false : stryMutAct_9fa48("116") ? true : (stryCov_9fa48("116", "117"), resp.error)) throw new Error(resp.error);
    return stryMutAct_9fa48("118") ? {} : (stryCov_9fa48("118"), {
      result: resp.data!,
      warnings: resp.warnings
    });
  }
}
export async function ipcSaveCoverConfig(config: CoverConfigInput): Promise<CoverConfig> {
  if (stryMutAct_9fa48("119")) {
    {}
  } else {
    stryCov_9fa48("119");
    const resp = await ipc<ApiResponse<CoverConfig>>(stryMutAct_9fa48("120") ? "" : (stryCov_9fa48("120"), 'save_cover_config'), stryMutAct_9fa48("121") ? {} : (stryCov_9fa48("121"), {
      config
    }));
    if (stryMutAct_9fa48("123") ? false : stryMutAct_9fa48("122") ? true : (stryCov_9fa48("122", "123"), resp.error)) throw new Error(resp.error);
    return resp.data!;
  }
}
export async function ipcGenerateCoverPdf(projectId: string): Promise<string> {
  if (stryMutAct_9fa48("124")) {
    {}
  } else {
    stryCov_9fa48("124");
    const resp = await ipc<ApiResponse<string>>(stryMutAct_9fa48("125") ? "" : (stryCov_9fa48("125"), 'generate_cover_pdf'), stryMutAct_9fa48("126") ? {} : (stryCov_9fa48("126"), {
      projectId
    }));
    if (stryMutAct_9fa48("128") ? false : stryMutAct_9fa48("127") ? true : (stryCov_9fa48("127", "128"), resp.error)) throw new Error(resp.error);
    return resp.data!;
  }
}
export async function ipcGetCoverTemplates(genre?: string): Promise<CoverTemplate[]> {
  if (stryMutAct_9fa48("129")) {
    {}
  } else {
    stryCov_9fa48("129");
    const resp = await ipc<ApiResponse<CoverTemplate[]>>(stryMutAct_9fa48("130") ? "" : (stryCov_9fa48("130"), 'get_cover_templates'), stryMutAct_9fa48("131") ? {} : (stryCov_9fa48("131"), {
      genre: stryMutAct_9fa48("132") ? genre && null : (stryCov_9fa48("132"), genre ?? null)
    }));
    if (stryMutAct_9fa48("134") ? false : stryMutAct_9fa48("133") ? true : (stryCov_9fa48("133", "134"), resp.error)) throw new Error(resp.error);
    return stryMutAct_9fa48("135") ? resp.data && [] : (stryCov_9fa48("135"), resp.data ?? (stryMutAct_9fa48("136") ? ["Stryker was here"] : (stryCov_9fa48("136"), [])));
  }
}
export async function ipcExportCoverImage(projectId: string, format: 'png' | 'jpeg', resolution: number): Promise<string> {
  if (stryMutAct_9fa48("137")) {
    {}
  } else {
    stryCov_9fa48("137");
    const resp = await ipc<ApiResponse<string>>(stryMutAct_9fa48("138") ? "" : (stryCov_9fa48("138"), 'export_cover_image'), stryMutAct_9fa48("139") ? {} : (stryCov_9fa48("139"), {
      projectId,
      format,
      resolution
    }));
    if (stryMutAct_9fa48("141") ? false : stryMutAct_9fa48("140") ? true : (stryCov_9fa48("140", "141"), resp.error)) throw new Error(resp.error);
    return resp.data!;
  }
}