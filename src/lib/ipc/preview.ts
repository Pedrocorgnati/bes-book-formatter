// IPC layer for preview — Tauri invoke() calls to Rust backend
import { ipc, unwrapResponse } from '$lib/utils/ipc';
import type { ApiResponse, LayoutIssue } from '$lib/types/interfaces';

// Annotation type (preview module domain)
export interface Annotation {
  id: string;
  projectId: string;
  pageNumber: number;
  xPercent: number;
  yPercent: number;
  annotationType: string;
  color: string;
  content: string;
  createdAt: string;
}

export interface AddAnnotationParams extends Record<string, unknown> {
  projectId: string;
  pageNumber: number;
  xPercent: number;
  yPercent: number;
  annotationType: string;
  color: string;
  content: string;
}

export async function ipcToggleDistractionFree(enabled: boolean): Promise<void> {
  await ipc<ApiResponse<null>>('toggle_distraction_free', { enabled });
}

export async function ipcGetAnnotations(projectId: string, pageNumber: number): Promise<Annotation[]> {
  const result = await ipc<ApiResponse<Annotation[]>>('get_annotations', { projectId, pageNumber });
  if (result.error) throw new Error(result.error);
  return result.data ?? [];
}

export async function ipcAddAnnotation(params: AddAnnotationParams): Promise<Annotation> {
  const result = await ipc<ApiResponse<Annotation>>('add_annotation', params);
  return unwrapResponse(result);
}

export async function ipcDeleteAnnotation(annotationId: string): Promise<void> {
  const result = await ipc<ApiResponse<null>>('delete_annotation', { annotationId });
  if (result.error) throw new Error(result.error);
}

// detect_orphans_widows is registered under preview.rs on the Rust side
export async function ipcDetectOrphansWidows(projectId: string): Promise<LayoutIssue[]> {
  const result = await ipc<ApiResponse<LayoutIssue[]>>('detect_orphans_widows', { projectId });
  if (result.error) throw new Error(result.error);
  return result.data ?? [];
}

export interface PreviewPageImage {
  pageNumber: number;
  imageBase64: string;
  widthPx: number;
  heightPx: number;
}

export interface PreviewPageResponse {
  pages: PreviewPageImage[];
  totalPages: number;
  renderMs: number;
}

export async function ipcRenderPreviewPage(
  projectId: string,
  page: number,
  zoom: number,
  spread: boolean
): Promise<PreviewPageResponse> {
  const result = await ipc<ApiResponse<PreviewPageResponse>>('render_preview_page', {
    projectId,
    page,
    zoom,
    spread,
  });
  return unwrapResponse(result);
}

/** Busca todas as anotações de um projeto (sem filtro por página) */
export async function ipcGetAllAnnotations(projectId: string): Promise<Annotation[]> {
  const result = await ipc<ApiResponse<Annotation[]>>('get_annotations', { projectId });
  if (result.error) throw new Error(result.error);
  return result.data ?? [];
}
