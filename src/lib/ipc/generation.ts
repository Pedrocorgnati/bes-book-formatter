// IPC layer for generation — Tauri invoke() calls to Rust backend
import { ipc, unwrapResponse } from '$lib/utils/ipc';
import type { ApiResponse, GenerationResult, PreflightResult, ValidationResult, StoredGenerationResult } from '$lib/types';

export async function ipcRunPreflight(projectId: string, format?: string): Promise<PreflightResult> {
  const result = await ipc<ApiResponse<PreflightResult>>('run_preflight', {
    projectId,
    format: format ?? null,
  });
  return unwrapResponse(result);
}

export async function ipcGenerateEpub(projectId: string, platform: string): Promise<GenerationResult> {
  const result = await ipc<ApiResponse<GenerationResult>>('generate_epub', { projectId, platform });
  return unwrapResponse(result);
}

export async function ipcGeneratePdfPrint(projectId: string, platform: string, pdfxProfile?: string): Promise<GenerationResult> {
  const result = await ipc<ApiResponse<GenerationResult>>('generate_pdf_print', {
    projectId,
    platform,
    pdfxProfile: pdfxProfile ?? null,
  });
  return unwrapResponse(result);
}

export async function ipcGeneratePdfEbook(projectId: string, platform: string): Promise<GenerationResult> {
  const result = await ipc<ApiResponse<GenerationResult>>('generate_pdf_ebook', { projectId, platform });
  return unwrapResponse(result);
}

export async function ipcGenerateDocx(projectId: string, platform?: string): Promise<GenerationResult> {
  const result = await ipc<ApiResponse<GenerationResult>>('generate_docx', {
    projectId,
    platform: platform ?? null,
  });
  return unwrapResponse(result);
}

export async function ipcGenerateHtml(projectId: string, platform?: string): Promise<GenerationResult> {
  const result = await ipc<ApiResponse<GenerationResult>>('generate_html', {
    projectId,
    platform: platform ?? null,
  });
  return unwrapResponse(result);
}

export async function ipcGetGenerationResults(projectId: string): Promise<StoredGenerationResult[]> {
  const result = await ipc<ApiResponse<StoredGenerationResult[]>>('get_generation_results', { projectId });
  return result.data ?? [];
}

export async function ipcRunEpubcheck(epubPath: string): Promise<ValidationResult> {
  const result = await ipc<ApiResponse<ValidationResult>>('run_epubcheck', { epubPath });
  return unwrapResponse(result);
}

export async function ipcCancelGeneration(projectId: string): Promise<void> {
  await ipc<ApiResponse<boolean>>('cancel_generation', { projectId });
}
