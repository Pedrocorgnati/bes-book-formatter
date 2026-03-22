// ==========================================================
// BES Book Formatter — Interfaces Compartilhadas (6 interfaces)
// ==========================================================

import type {
  BookConfigVersion,
  BookLanguage,
  Genre,
  IllustrationState,
  ManuscriptCompleteness,
  OutputFormat,
  Platform,
  UILanguage
} from './enums';

export interface BookProject {
  id: string;
  name: string;
  besRootPath: string;
  bookConfigPath: string | null;
  genre: Genre | null;
  language: BookLanguage;
  configVersion: BookConfigVersion | null;
  lastOpened: string | null; // ISO datetime string
  formatFilePath: string | null;
}

export interface BookConfig {
  version: BookConfigVersion;
  title: string;
  author: string;
  language: BookLanguage;
  genre: Genre;
  manuscriptRoot: string;
  outlineRoot: string | null;
  outputDir: string;
  platforms: Platform[];
  isbn: string | null;
}

export interface Illustration {
  id: string;
  projectId: string;
  placeholderName: string;
  description: string | null;
  state: IllustrationState;
  imagePath: string | null;
  validatedDpi: number | null;
  altText: string | null;
}

export interface GenerationResult {
  success: boolean;
  outputPath: string | null;
  format: OutputFormat;
  platform: Platform;
  errors: string[];
  warnings: string[];
  durationMs: number;
}

export interface ApiResponse<T> {
  data: T | null;
  error: string | null;
  warnings: string[];
}

export interface Pagination {
  page: number;
  perPage: number;
  total: number;
}

// Preferências do usuário (preferencesStore)
export interface UserPreferences {
  theme: 'light' | 'dark';
  uiLanguage: UILanguage;
  analyticsOptIn: boolean;
}

// Completude do manuscrito
export interface CompletenessResult {
  status: ManuscriptCompleteness;
  missingFields: string[];
  warnings: string[];
  completenessPercent: number;
}

// Estado dos sidecars
export interface SidecarInfo {
  name: string;
  version: string | null;
  available: boolean;
}

export interface SidecarStatus {
  typst: SidecarInfo;
  ghostscript: SidecarInfo;
  epubcheck: SidecarInfo;
  checkedAt: string; // ISO datetime
}
