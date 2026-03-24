// ==========================================================
// BES Book Formatter — Constantes de Timing Centralizadas
// ==========================================================

export const TIMING = {
  /** Debounce para auto-save de configurações tipográficas/layout (ms) */
  DEBOUNCE_CONFIG_SAVE: 500,

  /** Debounce para navegação/resize no visualizador de páginas (ms) */
  DEBOUNCE_PREVIEW_NAVIGATE: 300,

  /** Delay para limpar estado de cópia no DataTestOverlay (ms) */
  COPY_FEEDBACK_RESET: 1500,

  /** Delay para remover live region de acessibilidade do DOM (ms) */
  A11Y_LIVE_REGION_REMOVE: 3000,

  /** Delay para mover foco ao botão cancelar no ConfirmDialog (ms) */
  FOCUS_TRANSFER: 50,

  /** Aguardo para o sidecar backend iniciar (ms) */
  SIDECAR_STARTUP_WAIT: 500,

  // --- Toast durations ---

  /** Duração padrão de toasts de sucesso (ms) */
  TOAST_SUCCESS: 4000,

  /** Duração padrão de toasts de aviso (ms) */
  TOAST_WARNING: 6000,

  /** Duração padrão de toasts de info (ms) */
  TOAST_INFO: 4000,

  /** Duração 0 = toast persistente (não auto-dismiss) — usado em erros */
  TOAST_ERROR_PERSISTENT: 0,
} as const;

// --- UI Limits ---

/** Número máximo de toasts visíveis simultaneamente */
export const MAX_VISIBLE_TOASTS = 3;

/** Máximo de entradas em cache de páginas do preview (PageSpreadViewer) */
export const PREVIEW_PAGE_CACHE_SIZE = 10;

/** Limite padrão de projetos carregados na listagem do dashboard */
export const PROJECTS_LIST_LIMIT = 20;

/** Tamanho mínimo de texto de alt text para habilitar botão de salvar (caracteres) */
export const ALT_TEXT_MIN_LENGTH = 10;
