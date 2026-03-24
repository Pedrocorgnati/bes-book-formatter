// IPC layer for parser / manuscript — Tauri invoke() calls to Rust backend
import { ipc, unwrapResponse } from '$lib/utils/ipc';
import type { BookConfig } from '$lib/types/interfaces';
import type { ApiResponse, CompletenessResult } from '$lib/types/interfaces';

// Chapter entry from parsed manuscript AST
export interface ManuscriptChapter {
  slug: string;
  title: string;
  wordCount: number;
}

// Manuscript AST summary returned by parse_manuscript
export interface ManuscriptSummary {
  chapters: ManuscriptChapter[];
  totalWordCount: number;
  illustrationPlaceholders: number;
}

export async function ipcParseManuscript(projectId: string): Promise<ManuscriptSummary> {
  const result = await ipc<ApiResponse<ManuscriptSummary>>('parse_manuscript', { projectId });
  return unwrapResponse(result);
}

export async function ipcCalculateCompleteness(projectId: string): Promise<CompletenessResult> {
  const result = await ipc<ApiResponse<CompletenessResult>>('calculate_completeness', { projectId });
  return unwrapResponse(result);
}

/** Abre diálogo nativo para selecionar diretório */
export async function ipcSelectDirectory(): Promise<string | null> {
  return ipc<string | null>('select_directory');
}

/** Lê e valida o book.config do workspace BES */
export async function ipcReadBookConfig(path: string): Promise<BookConfig> {
  const result = await ipc<ApiResponse<BookConfig>>('read_book_config', { path });
  return unwrapResponse(result);
}
