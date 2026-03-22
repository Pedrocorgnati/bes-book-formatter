// projectStore.ts — estado dos projetos no app
import { writable, derived } from 'svelte/store';
import type { BookProject } from '$lib/types';

interface ProjectsState {
  list: BookProject[];
  current: BookProject | null;
  loading: boolean;
}

function createProjectsStore() {
  const { subscribe, set, update } = writable<ProjectsState>({
    list: [],
    current: null,
    loading: true,
  });

  return {
    subscribe,
    setProjects(projects: BookProject[]) {
      update(s => ({ ...s, list: projects, loading: false }));
    },
    setCurrent(project: BookProject | null) {
      update(s => ({ ...s, current: project }));
    },
    addProject(project: BookProject) {
      update(s => ({ ...s, list: [project, ...s.list] }));
    },
    removeProject(id: string) {
      update(s => ({
        ...s,
        list: s.list.filter(p => p.id !== id),
        current: s.current?.id === id ? null : s.current,
      }));
    },
    setLoading(loading: boolean) {
      update(s => ({ ...s, loading }));
    },
    reset() {
      set({ list: [], current: null, loading: false });
    },
  };
}

export const projectsStore = createProjectsStore();

// Compat: projetos recentes (últimos 10)
export const recentProjectsStore = derived(
  projectsStore,
  $s => $s.list.slice(0, 10)
);
