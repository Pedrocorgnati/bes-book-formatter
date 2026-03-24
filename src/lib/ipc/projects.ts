// IPC layer for projects — Tauri invoke() calls to Rust backend
import { ipc } from '$lib/utils/ipc';
import type { BookProject, ApiResponse } from '$lib/types/interfaces';
import { PROJECTS_LIST_LIMIT } from '$lib/constants/timing';

export async function ipcGetProjects(limit: number = PROJECTS_LIST_LIMIT): Promise<BookProject[]> {
  const result = await ipc<ApiResponse<BookProject[]>>('get_projects', { limit });
  return result.data ?? [];
}

export async function ipcGetProject(id: string): Promise<BookProject | null> {
  const result = await ipc<ApiResponse<BookProject>>('get_project', { id });
  return result.data ?? null;
}

export async function ipcImportProject(besRoot: string): Promise<BookProject | null> {
  const result = await ipc<ApiResponse<BookProject>>('import_project', { besRoot });
  if (result.error) {
    throw new Error(result.error);
  }
  return result.data ?? null;
}

export async function ipcDeleteProject(id: string): Promise<void> {
  const result = await ipc<ApiResponse<boolean>>('delete_project', { id });
  if (result.error) {
    throw new Error(result.error);
  }
}

export async function ipcInitDatabase(): Promise<void> {
  await ipc<void>('init_database');
}
