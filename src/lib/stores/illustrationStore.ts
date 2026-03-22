// illustrationStore.ts — catálogo de ilustrações do projeto atual
import { writable } from 'svelte/store';
import type { Illustration } from '$lib/types';

// Lista de ilustrações do projeto aberto
export const illustrationStore = writable<Illustration[]>([]);

// Loading state
export const illustrationsLoadingStore = writable<boolean>(false);
