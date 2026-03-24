// ==========================================================
// BES Book Formatter — Identificadores de Abas de UI
// Centraliza as strings de tab/section usadas em comparações
// ==========================================================

/** Abas do painel lateral do preview (PreviewSidebar) */
export const PREVIEW_TABS = {
  CHAPTERS: 'chapters',
  GALLERY:  'gallery',
  PROJECTS: 'projects',
} as const;

export type PreviewTab = typeof PREVIEW_TABS[keyof typeof PREVIEW_TABS];

/** Abas do seletor de formatos de geração (FormatSelector) */
export const FORMAT_TABS = {
  PRESET: 'preset',
  MANUAL: 'manual',
} as const;

export type FormatTab = typeof FORMAT_TABS[keyof typeof FORMAT_TABS];

/** Seções do editor de capa (CoverEditor) */
export const COVER_SECTIONS = {
  TEMPLATE: 'template',
  TEXT:     'text',
  DESIGN:   'design',
} as const;

export type CoverSection = typeof COVER_SECTIONS[keyof typeof COVER_SECTIONS];

/** Abas da página de configurações do projeto (project/[id]/settings) */
export const SETTINGS_TABS = {
  INTEGRATION: 'integration',
  PROGRESS:    'progress',
} as const;

export type SettingsTab = typeof SETTINGS_TABS[keyof typeof SETTINGS_TABS];

/** Filtros de ilustrações — inclui 'all' que não existe no enum IllustrationState */
export const ILLUSTRATION_FILTERS = {
  ALL:      'all',
  PENDING:  'pending',
  IMPORTED: 'imported',
  LINKED:   'linked',
  ERROR:    'error',
} as const;

export type IllustrationFilter = typeof ILLUSTRATION_FILTERS[keyof typeof ILLUSTRATION_FILTERS];
