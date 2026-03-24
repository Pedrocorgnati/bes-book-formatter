// @ts-nocheck
// IPC layer for typography — Tauri invoke() calls to Rust backend
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
import type { ApiResponse, TypographyConfig, DpiValidation, Illustration, FontInfo, LayoutIssue } from '$lib/types/interfaces';
export async function ipcGetTypographyConfig(projectId: string): Promise<TypographyConfig | null> {
  if (stryMutAct_9fa48("237")) {
    {}
  } else {
    stryCov_9fa48("237");
    const result = await ipc<ApiResponse<TypographyConfig>>(stryMutAct_9fa48("238") ? "" : (stryCov_9fa48("238"), 'get_typography_config'), stryMutAct_9fa48("239") ? {} : (stryCov_9fa48("239"), {
      projectId
    }));
    if (stryMutAct_9fa48("241") ? false : stryMutAct_9fa48("240") ? true : (stryCov_9fa48("240", "241"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("242") ? result.data && null : (stryCov_9fa48("242"), result.data ?? null);
  }
}
export async function ipcSetTypographyConfig(projectId: string, config: Partial<TypographyConfig>): Promise<TypographyConfig | null> {
  if (stryMutAct_9fa48("243")) {
    {}
  } else {
    stryCov_9fa48("243");
    const result = await ipc<ApiResponse<TypographyConfig>>(stryMutAct_9fa48("244") ? "" : (stryCov_9fa48("244"), 'set_typography_config'), stryMutAct_9fa48("245") ? {} : (stryCov_9fa48("245"), {
      projectId,
      config
    }));
    if (stryMutAct_9fa48("247") ? false : stryMutAct_9fa48("246") ? true : (stryCov_9fa48("246", "247"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("248") ? result.data && null : (stryCov_9fa48("248"), result.data ?? null);
  }
}
export async function ipcValidateIllustrationDpi(filePath: string): Promise<DpiValidation> {
  if (stryMutAct_9fa48("249")) {
    {}
  } else {
    stryCov_9fa48("249");
    const result = await ipc<ApiResponse<DpiValidation>>(stryMutAct_9fa48("250") ? "" : (stryCov_9fa48("250"), 'validate_illustration_dpi'), stryMutAct_9fa48("251") ? {} : (stryCov_9fa48("251"), {
      filePath
    }));
    if (stryMutAct_9fa48("253") ? false : stryMutAct_9fa48("252") ? true : (stryCov_9fa48("252", "253"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcProcessIllustration(illustrationId: string, filePath: string, projectId: string): Promise<Illustration | null> {
  if (stryMutAct_9fa48("254")) {
    {}
  } else {
    stryCov_9fa48("254");
    const result = await ipc<ApiResponse<Illustration>>(stryMutAct_9fa48("255") ? "" : (stryCov_9fa48("255"), 'process_illustration'), stryMutAct_9fa48("256") ? {} : (stryCov_9fa48("256"), {
      illustrationId,
      filePath,
      projectId
    }));
    if (stryMutAct_9fa48("258") ? false : stryMutAct_9fa48("257") ? true : (stryCov_9fa48("257", "258"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("259") ? result.data && null : (stryCov_9fa48("259"), result.data ?? null);
  }
}
export async function ipcListFonts(projectId: string): Promise<FontInfo[]> {
  if (stryMutAct_9fa48("260")) {
    {}
  } else {
    stryCov_9fa48("260");
    const result = await ipc<ApiResponse<FontInfo[]>>(stryMutAct_9fa48("261") ? "" : (stryCov_9fa48("261"), 'list_fonts'), stryMutAct_9fa48("262") ? {} : (stryCov_9fa48("262"), {
      projectId
    }));
    if (stryMutAct_9fa48("264") ? false : stryMutAct_9fa48("263") ? true : (stryCov_9fa48("263", "264"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("265") ? result.data && [] : (stryCov_9fa48("265"), result.data ?? (stryMutAct_9fa48("266") ? ["Stryker was here"] : (stryCov_9fa48("266"), [])));
  }
}
export async function ipcUploadFont(projectId: string, filePath: string): Promise<FontInfo> {
  if (stryMutAct_9fa48("267")) {
    {}
  } else {
    stryCov_9fa48("267");
    const result = await ipc<ApiResponse<FontInfo>>(stryMutAct_9fa48("268") ? "" : (stryCov_9fa48("268"), 'upload_font'), stryMutAct_9fa48("269") ? {} : (stryCov_9fa48("269"), {
      projectId,
      filePath
    }));
    if (stryMutAct_9fa48("271") ? false : stryMutAct_9fa48("270") ? true : (stryCov_9fa48("270", "271"), result.error)) throw new Error(result.error);
    return result.data!;
  }
}
export async function ipcDeleteCustomFont(projectId: string, fontName: string): Promise<void> {
  if (stryMutAct_9fa48("272")) {
    {}
  } else {
    stryCov_9fa48("272");
    const result = await ipc<ApiResponse<null>>(stryMutAct_9fa48("273") ? "" : (stryCov_9fa48("273"), 'delete_custom_font'), stryMutAct_9fa48("274") ? {} : (stryCov_9fa48("274"), {
      projectId,
      fontName
    }));
    if (stryMutAct_9fa48("276") ? false : stryMutAct_9fa48("275") ? true : (stryCov_9fa48("275", "276"), result.error)) throw new Error(result.error);
  }
}

// detect_orphans_widows is in commands/preview.rs (returns LayoutIssue[])
export async function ipcDetectOrphansWidows(projectId: string): Promise<LayoutIssue[]> {
  if (stryMutAct_9fa48("277")) {
    {}
  } else {
    stryCov_9fa48("277");
    const result = await ipc<ApiResponse<LayoutIssue[]>>(stryMutAct_9fa48("278") ? "" : (stryCov_9fa48("278"), 'detect_orphans_widows'), stryMutAct_9fa48("279") ? {} : (stryCov_9fa48("279"), {
      projectId
    }));
    if (stryMutAct_9fa48("281") ? false : stryMutAct_9fa48("280") ? true : (stryCov_9fa48("280", "281"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("282") ? result.data && [] : (stryCov_9fa48("282"), result.data ?? (stryMutAct_9fa48("283") ? ["Stryker was here"] : (stryCov_9fa48("283"), [])));
  }
}
export async function ipcListIllustrations(projectId: string): Promise<Illustration[]> {
  if (stryMutAct_9fa48("284")) {
    {}
  } else {
    stryCov_9fa48("284");
    const result = await ipc<ApiResponse<Illustration[]>>(stryMutAct_9fa48("285") ? "" : (stryCov_9fa48("285"), 'list_illustrations'), stryMutAct_9fa48("286") ? {} : (stryCov_9fa48("286"), {
      projectId
    }));
    if (stryMutAct_9fa48("288") ? false : stryMutAct_9fa48("287") ? true : (stryCov_9fa48("287", "288"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("289") ? result.data && [] : (stryCov_9fa48("289"), result.data ?? (stryMutAct_9fa48("290") ? ["Stryker was here"] : (stryCov_9fa48("290"), [])));
  }
}
export async function ipcUpdateIllustrationAltText(illustrationId: string, altText: string): Promise<Illustration | null> {
  if (stryMutAct_9fa48("291")) {
    {}
  } else {
    stryCov_9fa48("291");
    const result = await ipc<ApiResponse<Illustration>>(stryMutAct_9fa48("292") ? "" : (stryCov_9fa48("292"), 'update_illustration_alt_text'), stryMutAct_9fa48("293") ? {} : (stryCov_9fa48("293"), {
      illustrationId,
      altText
    }));
    if (stryMutAct_9fa48("295") ? false : stryMutAct_9fa48("294") ? true : (stryCov_9fa48("294", "295"), result.error)) throw new Error(result.error);
    return stryMutAct_9fa48("296") ? result.data && null : (stryCov_9fa48("296"), result.data ?? null);
  }
}