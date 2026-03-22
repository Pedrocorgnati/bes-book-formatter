// IPC layer for projects — Tauri invoke() calls to Rust backend
import { invoke } from '@tauri-apps/api/core';
import type { BookProject, ApiResponse } from '$lib/types/interfaces';

export async function ipcGetProjects(limit: number = 20): Promise<BookProject[]> {
  const result = await invoke<ApiResponse<BookProject[]>>('get_projects', { limit });
  return result.data ?? [];
}

export async function ipcGetProject(id: string): Promise<BookProject | null> {
  const result = await invoke<ApiResponse<BookProject>>('get_project', { id });
  return result.data ?? null;
}

export async function ipcImportProject(besRoot: string): Promise<BookProject | null> {
  const result = await invoke<ApiResponse<BookProject>>('import_project', { besRoot });
  if (result.error) {
    throw new Error(result.error);
  }
  return result.data ?? null;
}

export async function ipcDeleteProject(id: string): Promise<void> {
  const result = await invoke<ApiResponse<boolean>>('delete_project', { id });
  if (result.error) {
    throw new Error(result.error);
  }
}

export async function ipcInitDatabase(): Promise<void> {
  await invoke('init_database');
}
