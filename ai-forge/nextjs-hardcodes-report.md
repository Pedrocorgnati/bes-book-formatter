# Hardcodes Report — BES Book Formatter

## Hardcodes Encontrados e Corrigidos

### Status/State strings
| Valor | Ocorrências | Arquivos | Ação |
|-------|-------------|----------|------|
| `illustration.state === 'pending'` | 1 | IllustrationGallery | → `IllustrationState.PENDING` |
| `illustration.state === 'imported'` | 2 | IllustrationCard, IllustrationGallery | → `IllustrationState.IMPORTED` |
| `illustration.state === 'linked'` | 1 | IllustrationGallery | → `IllustrationState.LINKED` |
| `illustration.state === 'error'` | 2 | IllustrationCard, IllustrationGallery | → `IllustrationState.ERROR` |

### Rotas Hardcoded
| Valor | Arquivo | Ação |
|-------|---------|------|
| `goto('/')` | project/[id]/+layout.svelte (2×) | → `goto(ROUTES.HOME)` |
| `` goto(`/project/${project.id}`) `` | ImportWizard.svelte | → `goto(PROJECT_ROUTES.ROOT(id))` |

### Storage Keys
| Valor | Arquivo | Ação |
|-------|---------|------|
| `'bes_first_launch'` | +page.svelte (2×) | → `STORAGE_KEYS.FIRST_LAUNCH` |

### Magic Numbers — Timing
| Valor | Contexto | Ação |
|-------|----------|------|
| `4000` | toast success duration | → `TIMING.TOAST_SUCCESS` |
| `6000` | toast warning duration | → `TIMING.TOAST_WARNING` |
| `0` | toast error persistent | → `TIMING.TOAST_ERROR_PERSISTENT` |
| `3` | max visible toasts | → `MAX_VISIBLE_TOASTS` |
| `10` | preview page cache size | → `PREVIEW_PAGE_CACHE_SIZE` |
| `20` | projects list limit | → `PROJECTS_LIST_LIMIT` |
| `10` | alt text min length | → `ALT_TEXT_MIN_LENGTH` |

### Tab Identifiers
| Tipo | Valores | Componentes | Ação |
|------|---------|-------------|------|
| PreviewTab | `'chapters'`, `'gallery'`, `'projects'` | PreviewSidebar | → `PREVIEW_TABS.*` |
| FormatTab | `'preset'`, `'manual'` | FormatSelector | → `FORMAT_TABS.*` |
| CoverSection | `'template'`, `'text'`, `'design'` | CoverEditor | → `COVER_SECTIONS.*` |
| SettingsTab | `'integration'`, `'progress'` | settings/+page | → `SETTINGS_TABS.*` |
| IllustrationFilter | `'all'` | IllustrationGallery | → `ILLUSTRATION_FILTERS.ALL` |

### Toast Messages Hardcoded (PT sem i18n)
| Valor | Arquivo | Ação |
|-------|---------|------|
| `` `Preflight falhou: ${e}` `` | GenerationPanel | → `t('generation.preflightError')` |
| `` `Erro ao gerar ${format}: ${e}` `` | GenerationPanel | → `t('generation.generateFormatError')` |
| `'Um ou mais formatos falharam'` | GenerationPanel | → `t('generation.partialError')` |

### Preference DB Keys Hardcoded
| Valor | Arquivo | Ação |
|-------|---------|------|
| `'theme'`, `'ui_language'`, `'analytics_opt_in'` | ipc/preferences.ts | → `PREF_DB_KEYS` const |
| `'pt-BR'` (default) | ipc/preferences.ts | → `DEFAULT_UI_LANGUAGE` |
| `'light'` (default) | ipc/preferences.ts | → `DEFAULT_THEME` |

## Constantes Criadas/Atualizadas

| Arquivo | Exportações adicionadas |
|---------|------------------------|
| `src/lib/constants/storage-keys.ts` | `FIRST_LAUNCH` |
| `src/lib/constants/timing.ts` | `TOAST_SUCCESS`, `TOAST_WARNING`, `TOAST_INFO`, `TOAST_ERROR_PERSISTENT`, `MAX_VISIBLE_TOASTS`, `PREVIEW_PAGE_CACHE_SIZE`, `PROJECTS_LIST_LIMIT`, `ALT_TEXT_MIN_LENGTH` |
| `src/lib/constants/ui-tabs.ts` | **NOVO** — `PREVIEW_TABS`, `FORMAT_TABS`, `COVER_SECTIONS`, `SETTINGS_TABS`, `ILLUSTRATION_FILTERS` e types |
| `src/lib/constants/index.ts` | Re-export de `ui-tabs` |
| `src/lib/i18n/pt-BR.json` | `generation.preflightError`, `generation.generateFormatError`, `generation.partialError` |
| `src/lib/i18n/en-US.json` | idem |
| `src/lib/i18n/es-ES.json` | idem |
