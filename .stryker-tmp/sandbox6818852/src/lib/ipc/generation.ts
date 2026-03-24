// @ts-nocheck
// IPC layer for generation — Tauri invoke() calls to Rust backend
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
import type { ApiResponse, GenerationResult, PreflightResult, ValidationResult, StoredGenerationResult } from '$lib/types';
export async function ipcRunPreflight(projectId: string, format?: string): Promise<PreflightResult> {
  if (stryMutAct_9fa48("142")) {
    {}
  } else {
    stryCov_9fa48("142");
    const result = await ipc<ApiResponse<PreflightResult>>(stryMutAct_9fa48("143") ? "" : (stryCov_9fa48("143"), 'run_preflight'), stryMutAct_9fa48("144") ? {} : (stryCov_9fa48("144"), {
      projectId,
      format: stryMutAct_9fa48("145") ? format && null : (stryCov_9fa48("145"), format ?? null)
    }));
    if (stryMutAct_9fa48("147") ? false : stryMutAct_9fa48("146") ? true : (stryCov_9fa48("146", "147"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGenerateEpub(projectId: string, platform: string): Promise<GenerationResult> {
  if (stryMutAct_9fa48("148")) {
    {}
  } else {
    stryCov_9fa48("148");
    const result = await ipc<ApiResponse<GenerationResult>>(stryMutAct_9fa48("149") ? "" : (stryCov_9fa48("149"), 'generate_epub'), stryMutAct_9fa48("150") ? {} : (stryCov_9fa48("150"), {
      projectId,
      platform
    }));
    if (stryMutAct_9fa48("152") ? false : stryMutAct_9fa48("151") ? true : (stryCov_9fa48("151", "152"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGeneratePdfPrint(projectId: string, platform: string, pdfxProfile?: string): Promise<GenerationResult> {
  if (stryMutAct_9fa48("153")) {
    {}
  } else {
    stryCov_9fa48("153");
    const result = await ipc<ApiResponse<GenerationResult>>(stryMutAct_9fa48("154") ? "" : (stryCov_9fa48("154"), 'generate_pdf_print'), stryMutAct_9fa48("155") ? {} : (stryCov_9fa48("155"), {
      projectId,
      platform,
      pdfxProfile: stryMutAct_9fa48("156") ? pdfxProfile && null : (stryCov_9fa48("156"), pdfxProfile ?? null)
    }));
    if (stryMutAct_9fa48("158") ? false : stryMutAct_9fa48("157") ? true : (stryCov_9fa48("157", "158"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGeneratePdfEbook(projectId: string, platform: string): Promise<GenerationResult> {
  if (stryMutAct_9fa48("159")) {
    {}
  } else {
    stryCov_9fa48("159");
    const result = await ipc<ApiResponse<GenerationResult>>(stryMutAct_9fa48("160") ? "" : (stryCov_9fa48("160"), 'generate_pdf_ebook'), stryMutAct_9fa48("161") ? {} : (stryCov_9fa48("161"), {
      projectId,
      platform
    }));
    if (stryMutAct_9fa48("163") ? false : stryMutAct_9fa48("162") ? true : (stryCov_9fa48("162", "163"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGenerateDocx(projectId: string, platform?: string): Promise<GenerationResult> {
  if (stryMutAct_9fa48("164")) {
    {}
  } else {
    stryCov_9fa48("164");
    const result = await ipc<ApiResponse<GenerationResult>>(stryMutAct_9fa48("165") ? "" : (stryCov_9fa48("165"), 'generate_docx'), stryMutAct_9fa48("166") ? {} : (stryCov_9fa48("166"), {
      projectId,
      platform: stryMutAct_9fa48("167") ? platform && null : (stryCov_9fa48("167"), platform ?? null)
    }));
    if (stryMutAct_9fa48("169") ? false : stryMutAct_9fa48("168") ? true : (stryCov_9fa48("168", "169"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGenerateHtml(projectId: string, platform?: string): Promise<GenerationResult> {
  if (stryMutAct_9fa48("170")) {
    {}
  } else {
    stryCov_9fa48("170");
    const result = await ipc<ApiResponse<GenerationResult>>(stryMutAct_9fa48("171") ? "" : (stryCov_9fa48("171"), 'generate_html'), stryMutAct_9fa48("172") ? {} : (stryCov_9fa48("172"), {
      projectId,
      platform: stryMutAct_9fa48("173") ? platform && null : (stryCov_9fa48("173"), platform ?? null)
    }));
    if (stryMutAct_9fa48("175") ? false : stryMutAct_9fa48("174") ? true : (stryCov_9fa48("174", "175"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcGetGenerationResults(projectId: string): Promise<StoredGenerationResult[]> {
  if (stryMutAct_9fa48("176")) {
    {}
  } else {
    stryCov_9fa48("176");
    const result = await ipc<ApiResponse<StoredGenerationResult[]>>(stryMutAct_9fa48("177") ? "" : (stryCov_9fa48("177"), 'get_generation_results'), stryMutAct_9fa48("178") ? {} : (stryCov_9fa48("178"), {
      projectId
    }));
    return stryMutAct_9fa48("179") ? result.data && [] : (stryCov_9fa48("179"), result.data ?? (stryMutAct_9fa48("180") ? ["Stryker was here"] : (stryCov_9fa48("180"), [])));
  }
}
export async function ipcRunEpubcheck(epubPath: string): Promise<ValidationResult> {
  if (stryMutAct_9fa48("181")) {
    {}
  } else {
    stryCov_9fa48("181");
    const result = await ipc<ApiResponse<ValidationResult>>(stryMutAct_9fa48("182") ? "" : (stryCov_9fa48("182"), 'run_epubcheck'), stryMutAct_9fa48("183") ? {} : (stryCov_9fa48("183"), {
      epubPath
    }));
    if (stryMutAct_9fa48("185") ? false : stryMutAct_9fa48("184") ? true : (stryCov_9fa48("184", "185"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcCancelGeneration(projectId: string): Promise<void> {
  if (stryMutAct_9fa48("186")) {
    {}
  } else {
    stryCov_9fa48("186");
    await ipc<ApiResponse<boolean>>(stryMutAct_9fa48("187") ? "" : (stryCov_9fa48("187"), 'cancel_generation'), stryMutAct_9fa48("188") ? {} : (stryCov_9fa48("188"), {
      projectId
    }));
  }
}