// sidecarStore.ts — disponibilidade dos sidecars (Sharp, Typst, etc.)
import { writable, derived } from 'svelte/store';

export interface SidecarAvailability {
  sharp: boolean;
  typst: boolean;
  ghostscript: boolean;
  epubcheck: boolean;
  checkedAt: string | null;
}

const defaultAvailability: SidecarAvailability = {
  sharp: false,
  typst: false,
  ghostscript: false,
  epubcheck: false,
  checkedAt: null,
};

export const sidecarStore = writable<SidecarAvailability>(defaultAvailability);

// Derived: Sharp disponível para processamento avançado de imagens
export const sharpAvailable = derived(sidecarStore, ($s) => $s.sharp);

// Derived: geração de PDF disponível
export const pdfGenerationAvailable = derived(sidecarStore, ($s) => $s.typst);

// Derived: validação EPUB disponível
export const epubValidationAvailable = derived(sidecarStore, ($s) => $s.epubcheck);
