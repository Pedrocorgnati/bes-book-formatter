// IPC layer for illustrations — Tauri invoke() calls to Rust backend
import { ipc, unwrapResponse } from '$lib/utils/ipc';
import type { ApiResponse, DpiValidation, Illustration } from '$lib/types/interfaces';

export async function ipcValidateIllustrationDpi(filePath: string): Promise<DpiValidation> {
  const result = await ipc<ApiResponse<DpiValidation>>('validate_illustration_dpi', { filePath });
  return unwrapResponse(result);
}

export async function ipcProcessIllustration(
  illustrationId: string,
  filePath: string,
  projectId: string
): Promise<Illustration | null> {
  const result = await ipc<ApiResponse<Illustration>>('process_illustration', {
    illustrationId,
    filePath,
    projectId,
  });
  if (result.error) throw new Error(result.error);
  return result.data ?? null;
}

export async function ipcListIllustrations(projectId: string): Promise<Illustration[]> {
  const result = await ipc<ApiResponse<Illustration[]>>('list_illustrations', { projectId });
  if (result.error) throw new Error(result.error);
  return result.data ?? [];
}

export async function ipcUpdateIllustrationAltText(
  illustrationId: string,
  altText: string
): Promise<Illustration | null> {
  const result = await ipc<ApiResponse<Illustration>>('update_illustration_alt_text', {
    illustrationId,
    altText,
  });
  if (result.error) throw new Error(result.error);
  return result.data ?? null;
}
