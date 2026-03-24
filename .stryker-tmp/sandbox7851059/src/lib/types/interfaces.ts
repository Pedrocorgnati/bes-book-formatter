// @ts-nocheck
// ==========================================================
// BES Book Formatter — Interfaces Compartilhadas (6 interfaces)
// ==========================================================

import type { BookConfigVersion, BookLanguage, Genre, IllustrationState, OutputFormat, Platform, UILanguage } from './enums';
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
  completenessLevel: string | null;
  completenessScore: number | null;
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
  pageDimensions: PageDimensions | null;
  typography: TypographyDefaults | null;
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
  widthPx: number | null;
  heightPx: number | null;
  colorSpace: string | null;
  createdAt: string;
  updatedAt: string;
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
export interface PageDimensions {
  widthInches: number;
  heightInches: number;
  marginTop: number;
  marginBottom: number;
  marginInner: number;
  marginOuter: number;
}
export interface TypographyDefaults {
  bodyFont: string;
  headingFont: string;
  codeFont: string | null;
  bodySizePt: number;
  lineHeight: number;
}

// Completude do manuscrito
export interface CompletenessResult {
  score: number;
  level: string;
  totalChapters: number;
  chaptersWithContent: number;
  emptyChapters: string[];
  placeholderCount: number;
  warnings: string[];
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

// Typography configuration (Rock-2 — module-3)
export interface TypographyConfig {
  id: string;
  projectId: string;
  fontBody: string;
  fontHeading: string;
  fontCode: string | null;
  fontSizeBody: number;
  fontSizeH1: number;
  fontSizeH2: number;
  fontSizeH3: number;
  fontSizeH4: number;
  leading: number;
  paragraphIndent: number;
  tracking: number;
  kerning: boolean;
  justification: boolean;
  hyphenation: boolean;
  hyphenationLanguage: string;
  orphanControl: number;
  widowControl: number;
  dropCapStyle: string;
  ornamentStyle: string;
  baselineGrid: number;
  genrePreset: string;
  customOverrides: Record<string, unknown>;
  pageWidth: number;
  pageHeight: number;
  marginTop: number;
  marginBottom: number;
  marginInner: number;
  marginOuter: number;
  chapterStart: string;
  illustrationMissingMode: 'placeholder_visual' | 'remove_space' | 'block_generation';
  createdAt: string;
  updatedAt: string;
}

// Font entry in the font catalog
export interface FontInfo {
  name: string;
  path: string;
  isBundled: boolean;
}

// DPI validation result for illustrations
export interface DpiValidation {
  dpi: number;
  adequate: boolean;
  warning: string | null;
}

// Orphan/widow detection result from detect_orphans_widows IPC (module-3 TASK-3)
export interface TypoIssue {
  page: number;
  issueType: string; // "orphan" | "widow"
  textExcerpt: string;
  suggestion: string;
}

// Layout issue from preview/detect_orphans_widows IPC (Rock-4 LayoutIssue struct)
export interface LayoutIssue {
  issueType: string; // "orphan" | "widow" | "short_page"
  page: number;
  description: string;
}

// Generation module types (module-4)

export interface ChecklistItem {
  id: string;
  message: string;
  files: string[] | null;
}
export interface PreflightResult {
  passed: boolean;
  blockers: ChecklistItem[];
  warnings: ChecklistItem[];
}
export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
export interface StoredGenerationResult {
  id: string;
  projectId: string;
  format: string;
  platform: string;
  outputPath: string | null;
  fileSizeBytes: number | null;
  durationMs: number | null;
  status: string; // 'success' | 'error' | 'cancelled'
  errors: string; // JSON array string
  warnings: string; // JSON array string
  createdAt: string;
}
export interface GenOptions {
  format: string;
  platform: string;
  paperColor?: string | null;
  dpi?: number | null;
  includeBleed?: boolean | null;
  pdfxProfile?: string | null;
}
export interface FormatSelection {
  formats: string[];
  platform: string;
  preset: string | null;
}

// IllustrationFull is an alias — Illustration already contains all fields.

// Cover Design (module-7)

export interface CoverConfig {
  id: string;
  projectId: string;
  templateId: string;
  genre: string;
  platform: 'amazon-kdp' | 'ingram' | 'generic';
  titleOverride: string | null;
  subtitle: string | null;
  authorOverride: string | null;
  backCoverText: string;
  primaryColor: string;
  secondaryColor: string;
  fontTitle: string;
  fontAuthor: string;
  coverImagePath: string | null;
  coverImageOriginal: string | null;
  coverImageDpi: number | null;
  pageCount: number;
  spineWidthMm: number;
  paperType: 'white' | 'cream';
  createdAt: string;
  updatedAt: string;
}
export interface CoverConfigInput {
  projectId: string;
  templateId?: string | null;
  genre?: string | null;
  platform?: 'amazon-kdp' | 'ingram' | 'generic' | null;
  titleOverride?: string | null;
  subtitle?: string | null;
  authorOverride?: string | null;
  backCoverText?: string | null;
  primaryColor?: string | null;
  secondaryColor?: string | null;
  fontTitle?: string | null;
  fontAuthor?: string | null;
  coverImagePath?: string | null;
  pageCount?: number | null;
  paperType?: 'white' | 'cream' | null;
}
export interface CoverTemplate {
  id: string;
  genre: string;
  name: string;
  description: string;
  primaryColor: string;
  secondaryColor: string;
  tags: string[];
  typstTemplate: string;
}
export interface SpineWidthResult {
  spineWidthMm: number;
  spineWidthInches: number;
  pageCount: number;
  platform: string;
  paperType: string;
}