// generationStore.ts — estado do módulo de geração (module-4)
import { writable } from 'svelte/store';
import type { StoredGenerationResult, PreflightResult, GenerationResult } from '$lib/types';
import { GenerationStatus } from '$lib/types/enums';

export { GenerationStatus };

interface GenerationState {
  status: GenerationStatus;
  activeFormat: string | null;
  activePlatform: string | null;
  preflight: PreflightResult | null;
  lastResult: GenerationResult | null;
  history: StoredGenerationResult[];
  historyLoading: boolean;
  error: string | null;
}

function createGenerationStore() {
  const { subscribe, set, update } = writable<GenerationState>({
    status: GenerationStatus.IDLE,
    activeFormat: null,
    activePlatform: null,
    preflight: null,
    lastResult: null,
    history: [],
    historyLoading: false,
    error: null,
  });

  return {
    subscribe,
    startPreflight(format: string, platform: string) {
      update(s => ({ ...s, status: GenerationStatus.PREFLIGHT, activeFormat: format, activePlatform: platform, error: null }));
    },
    setPreflight(result: PreflightResult) {
      update(s => ({ ...s, preflight: result }));
    },
    startGeneration() {
      update(s => ({ ...s, status: GenerationStatus.GENERATING, error: null }));
    },
    setResult(result: GenerationResult) {
      update(s => ({
        ...s,
        status: result.success ? GenerationStatus.DONE : GenerationStatus.ERROR,
        lastResult: result,
        error: result.success ? null : (result.errors[0] ?? 'Erro desconhecido'),
      }));
    },
    setError(msg: string) {
      update(s => ({ ...s, status: GenerationStatus.ERROR, error: msg }));
    },
    setHistory(items: StoredGenerationResult[]) {
      update(s => ({ ...s, history: items, historyLoading: false }));
    },
    setHistoryLoading(loading: boolean) {
      update(s => ({ ...s, historyLoading: loading }));
    },
    reset() {
      update(s => ({ ...s, status: GenerationStatus.IDLE, activeFormat: null, activePlatform: null, preflight: null, lastResult: null, error: null }));
    },
  };
}

export const generationStore = createGenerationStore();
