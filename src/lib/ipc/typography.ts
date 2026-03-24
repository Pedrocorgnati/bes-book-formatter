// IPC layer for typography — Tauri invoke() calls to Rust backend
// Note: illustration IPC → $lib/ipc/illustrations | preview IPC → $lib/ipc/preview
import { ipc, unwrapResponse } from '$lib/utils/ipc';
import type { ApiResponse, TypographyConfig, FontInfo } from '$lib/types/interfaces';

export async function ipcGetTypographyConfig(projectId: string): Promise<TypographyConfig | null> {
  const result = await ipc<ApiResponse<TypographyConfig>>('get_typography_config', { projectId });
  if (result.error) throw new Error(result.error);
  return result.data ?? null;
}

export async function ipcSetTypographyConfig(
  projectId: string,
  config: Partial<TypographyConfig>
): Promise<TypographyConfig | null> {
  const result = await ipc<ApiResponse<TypographyConfig>>('set_typography_config', {
    projectId,
    config,
  });
  if (result.error) throw new Error(result.error);
  return result.data ?? null;
}

export async function ipcListFonts(projectId: string): Promise<FontInfo[]> {
  const result = await ipc<ApiResponse<FontInfo[]>>('list_fonts', { projectId });
  if (result.error) throw new Error(result.error);
  return result.data ?? [];
}

export async function ipcUploadFont(projectId: string, filePath: string): Promise<FontInfo> {
  const result = await ipc<ApiResponse<FontInfo>>('upload_font', { projectId, filePath });
  return unwrapResponse(result);
}

export async function ipcDeleteCustomFont(projectId: string, fontName: string): Promise<void> {
  const result = await ipc<ApiResponse<null>>('delete_custom_font', { projectId, fontName });
  if (result.error) throw new Error(result.error);
}

export async function ipcGenerateToc(projectId: string): Promise<string> {
  const result = await ipc<ApiResponse<string>>('generate_toc', { projectId });
  if (result.error) throw new Error(result.error);
  return result.data ?? '';
}
