// IPC layer for cover design (module-7) — Tauri invoke() calls
import { ipc } from '$lib/utils/ipc';
import type {
  ApiResponse,
  CoverConfig,
  CoverConfigInput,
  CoverTemplate,
  SpineWidthResult,
} from '$lib/types/interfaces';

export async function ipcGetCoverConfig(projectId: string): Promise<CoverConfig | null> {
  const resp = await ipc<ApiResponse<CoverConfig | null>>('get_cover_config', { projectId });
  if (resp.error) throw new Error(resp.error);
  return resp.data ?? null;
}

export async function ipcCalculateSpineWidth(
  projectId: string,
  platform: string,
  paperType: string
): Promise<{ result: SpineWidthResult; warnings: string[] }> {
  const resp = await ipc<ApiResponse<SpineWidthResult>>('calculate_spine_width', {
    projectId,
    platform,
    paperType,
  });
  if (resp.error) throw new Error(resp.error);
  return { result: resp.data!, warnings: resp.warnings };
}

export async function ipcSaveCoverConfig(config: CoverConfigInput): Promise<CoverConfig> {
  const resp = await ipc<ApiResponse<CoverConfig>>('save_cover_config', { config });
  if (resp.error) throw new Error(resp.error);
  return resp.data!;
}

export async function ipcGenerateCoverPdf(projectId: string): Promise<string> {
  const resp = await ipc<ApiResponse<string>>('generate_cover_pdf', { projectId });
  if (resp.error) throw new Error(resp.error);
  return resp.data!;
}

export async function ipcGetCoverTemplates(genre?: string): Promise<CoverTemplate[]> {
  const resp = await ipc<ApiResponse<CoverTemplate[]>>('get_cover_templates', {
    genre: genre ?? null,
  });
  if (resp.error) throw new Error(resp.error);
  return resp.data ?? [];
}

export async function ipcExportCoverImage(
  projectId: string,
  format: 'png' | 'jpeg',
  resolution: number
): Promise<string> {
  const resp = await ipc<ApiResponse<string>>('export_cover_image', {
    projectId,
    format,
    resolution,
  });
  if (resp.error) throw new Error(resp.error);
  return resp.data!;
}
