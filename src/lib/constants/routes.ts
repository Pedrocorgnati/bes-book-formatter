// ==========================================================
// BES Book Formatter — Rotas SvelteKit Centralizadas
// ==========================================================

export const ROUTES = {
  HOME: '/',
  IMPORT: '/import',
  SETTINGS: '/settings',
} as const;

/** Rotas dinâmicas de projeto (requerem ID) */
export const PROJECT_ROUTES = {
  ROOT: (id: string) => `/project/${id}` as const,
  TYPOGRAPHY: (id: string) => `/project/${id}/typography` as const,
  ILLUSTRATIONS: (id: string) => `/project/${id}/illustrations` as const,
  OUTPUT: (id: string) => `/project/${id}/output` as const,
  PREVIEW: (id: string) => `/project/${id}/preview` as const,
  COVER: (id: string) => `/project/${id}/cover` as const,
  INTEGRATION: (id: string) => `/project/${id}/integration` as const,
  SETTINGS: (id: string) => `/project/${id}/settings` as const,
} as const;

export type Route = typeof ROUTES[keyof typeof ROUTES];
