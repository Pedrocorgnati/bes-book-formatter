// IPC layer for BES integration — Tauri invoke() calls to Rust backend
import { ipc, unwrapResponse } from '$lib/utils/ipc';
import type {
  BesWorkspaceInfo,
  BesDocuments,
  BesMetadata,
  EditorialProgress,
} from '$lib/types/bes';
import type { ApiResponse } from '$lib/types';

export async function ipcValidateBesWorkspace(workspacePath: string): Promise<BesWorkspaceInfo> {
  const result = await ipc<ApiResponse<BesWorkspaceInfo>>('validate_bes_workspace', {
    workspacePath,
  });
  return unwrapResponse(result);
}

export async function ipcReadBesDocs(projectId: string, workspacePath: string): Promise<BesDocuments> {
  const result = await ipc<ApiResponse<BesDocuments>>('read_bes_docs', {
    projectId,
    workspacePath,
  });
  return unwrapResponse(result);
}

export async function ipcGetBesMetadata(projectId: string, workspacePath: string): Promise<BesMetadata | null> {
  const result = await ipc<ApiResponse<BesMetadata>>('get_bes_metadata', {
    projectId,
    workspacePath,
  });
  if (result.error) throw new Error(result.error);
  return result.data ?? null;
}

export async function ipcInvalidateBesCache(projectId: string): Promise<void> {
  const result = await ipc<ApiResponse<boolean>>('invalidate_bes_cache', { projectId });
  if (result.error) throw new Error(result.error);
}

export async function ipcSyncEditorialProgress(
  projectId: string,
  workspacePath: string,
  projectName: string,
): Promise<EditorialProgress> {
  const result = await ipc<ApiResponse<EditorialProgress>>('sync_editorial_progress', {
    projectId,
    workspacePath,
    projectName,
  });
  return unwrapResponse(result);
}

export async function ipcUpdateEditorialF10(
  projectId: string,
  workspacePath: string,
  projectName: string,
  formatsGenerated: string[],
  outputPath: string,
): Promise<EditorialProgress> {
  const result = await ipc<ApiResponse<EditorialProgress>>('update_editorial_f10', {
    projectId,
    workspacePath,
    projectName,
    formatsGenerated,
    outputPath,
  });
  return unwrapResponse(result);
}
