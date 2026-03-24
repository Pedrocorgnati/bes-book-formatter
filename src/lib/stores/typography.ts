// typographyStore.ts — configuração tipográfica do projeto atual
import { writable, derived } from 'svelte/store';
import type { TypographyConfig } from '$lib/types/interfaces';

// Configuração tipográfica do projeto aberto
export const typographyStore = writable<TypographyConfig | null>(null);

// Loading state (enquanto carrega/salva via IPC)
export const typographyLoadingStore = writable<boolean>(false);

// Derived: preset atual selecionado
export const currentGenrePreset = derived(
  typographyStore,
  ($config) => $config?.genrePreset ?? 'nonfiction'
);

// Derived: página tem formato personalizado?
export const isCustomPageFormat = derived(
  typographyStore,
  ($config) => {
    if (!$config) return false;
    const stdFormats = [
      { w: 6.0, h: 9.0 },   // 6×9
      { w: 5.5, h: 8.5 },   // 5.5×8.5
      { w: 7.0, h: 10.0 },  // 7×10
      { w: 8.27, h: 11.69 }, // A4
      { w: 5.83, h: 8.27 }, // A5
      { w: 8.5, h: 11.0 },  // Letter
    ];
    return !stdFormats.some(
      (f) =>
        Math.abs(f.w - $config.pageWidth) < 0.01 &&
        Math.abs(f.h - $config.pageHeight) < 0.01
    );
  }
);
