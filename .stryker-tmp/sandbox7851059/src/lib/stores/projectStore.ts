// @ts-nocheck
// projectStore.ts — estado dos projetos no app
function stryNS_9fa48() {
  var g = typeof globalThis === 'object' && globalThis && globalThis.Math === Math && globalThis || new Function("return this")();
  var ns = g.__stryker__ || (g.__stryker__ = {});
  if (ns.activeMutant === undefined && g.process && g.process.env && g.process.env.__STRYKER_ACTIVE_MUTANT__) {
    ns.activeMutant = g.process.env.__STRYKER_ACTIVE_MUTANT__;
  }
  function retrieveNS() {
    return ns;
  }
  stryNS_9fa48 = retrieveNS;
  return retrieveNS();
}
stryNS_9fa48();
function stryCov_9fa48() {
  var ns = stryNS_9fa48();
  var cov = ns.mutantCoverage || (ns.mutantCoverage = {
    static: {},
    perTest: {}
  });
  function cover() {
    var c = cov.static;
    if (ns.currentTestId) {
      c = cov.perTest[ns.currentTestId] = cov.perTest[ns.currentTestId] || {};
    }
    var a = arguments;
    for (var i = 0; i < a.length; i++) {
      c[a[i]] = (c[a[i]] || 0) + 1;
    }
  }
  stryCov_9fa48 = cover;
  cover.apply(null, arguments);
}
function stryMutAct_9fa48(id) {
  var ns = stryNS_9fa48();
  function isActive(id) {
    if (ns.activeMutant === id) {
      if (ns.hitCount !== void 0 && ++ns.hitCount > ns.hitLimit) {
        throw new Error('Stryker: Hit count limit reached (' + ns.hitCount + ')');
      }
      return true;
    }
    return false;
  }
  stryMutAct_9fa48 = isActive;
  return isActive(id);
}
import { writable, derived } from 'svelte/store';
import type { BookProject } from '$lib/types';
interface ProjectsState {
  list: BookProject[];
  current: BookProject | null;
  loading: boolean;
  error: string | null;
}
function createProjectsStore() {
  if (stryMutAct_9fa48("381")) {
    {}
  } else {
    stryCov_9fa48("381");
    const {
      subscribe,
      set,
      update
    } = writable<ProjectsState>(stryMutAct_9fa48("382") ? {} : (stryCov_9fa48("382"), {
      list: stryMutAct_9fa48("383") ? ["Stryker was here"] : (stryCov_9fa48("383"), []),
      current: null,
      loading: stryMutAct_9fa48("384") ? false : (stryCov_9fa48("384"), true),
      error: null
    }));
    return stryMutAct_9fa48("385") ? {} : (stryCov_9fa48("385"), {
      subscribe,
      setProjects(projects: BookProject[]) {
        if (stryMutAct_9fa48("386")) {
          {}
        } else {
          stryCov_9fa48("386");
          update(stryMutAct_9fa48("387") ? () => undefined : (stryCov_9fa48("387"), s => stryMutAct_9fa48("388") ? {} : (stryCov_9fa48("388"), {
            ...s,
            list: projects,
            loading: stryMutAct_9fa48("389") ? true : (stryCov_9fa48("389"), false)
          })));
        }
      },
      setCurrent(project: BookProject | null) {
        if (stryMutAct_9fa48("390")) {
          {}
        } else {
          stryCov_9fa48("390");
          update(stryMutAct_9fa48("391") ? () => undefined : (stryCov_9fa48("391"), s => stryMutAct_9fa48("392") ? {} : (stryCov_9fa48("392"), {
            ...s,
            current: project
          })));
        }
      },
      addProject(project: BookProject) {
        if (stryMutAct_9fa48("393")) {
          {}
        } else {
          stryCov_9fa48("393");
          update(stryMutAct_9fa48("394") ? () => undefined : (stryCov_9fa48("394"), s => stryMutAct_9fa48("395") ? {} : (stryCov_9fa48("395"), {
            ...s,
            list: stryMutAct_9fa48("396") ? [] : (stryCov_9fa48("396"), [project, ...s.list])
          })));
        }
      },
      removeProject(id: string) {
        if (stryMutAct_9fa48("397")) {
          {}
        } else {
          stryCov_9fa48("397");
          update(stryMutAct_9fa48("398") ? () => undefined : (stryCov_9fa48("398"), s => stryMutAct_9fa48("399") ? {} : (stryCov_9fa48("399"), {
            ...s,
            list: stryMutAct_9fa48("400") ? s.list : (stryCov_9fa48("400"), s.list.filter(stryMutAct_9fa48("401") ? () => undefined : (stryCov_9fa48("401"), p => stryMutAct_9fa48("404") ? p.id === id : stryMutAct_9fa48("403") ? false : stryMutAct_9fa48("402") ? true : (stryCov_9fa48("402", "403", "404"), p.id !== id)))),
            current: (stryMutAct_9fa48("407") ? s.current?.id !== id : stryMutAct_9fa48("406") ? false : stryMutAct_9fa48("405") ? true : (stryCov_9fa48("405", "406", "407"), (stryMutAct_9fa48("408") ? s.current.id : (stryCov_9fa48("408"), s.current?.id)) === id)) ? null : s.current
          })));
        }
      },
      setLoading(loading: boolean) {
        if (stryMutAct_9fa48("409")) {
          {}
        } else {
          stryCov_9fa48("409");
          update(stryMutAct_9fa48("410") ? () => undefined : (stryCov_9fa48("410"), s => stryMutAct_9fa48("411") ? {} : (stryCov_9fa48("411"), {
            ...s,
            loading
          })));
        }
      },
      setError(error: string | null) {
        if (stryMutAct_9fa48("412")) {
          {}
        } else {
          stryCov_9fa48("412");
          update(stryMutAct_9fa48("413") ? () => undefined : (stryCov_9fa48("413"), s => stryMutAct_9fa48("414") ? {} : (stryCov_9fa48("414"), {
            ...s,
            error
          })));
        }
      },
      reset() {
        if (stryMutAct_9fa48("415")) {
          {}
        } else {
          stryCov_9fa48("415");
          set(stryMutAct_9fa48("416") ? {} : (stryCov_9fa48("416"), {
            list: stryMutAct_9fa48("417") ? ["Stryker was here"] : (stryCov_9fa48("417"), []),
            current: null,
            loading: stryMutAct_9fa48("418") ? true : (stryCov_9fa48("418"), false),
            error: null
          }));
        }
      }
    });
  }
}
export const projectsStore = createProjectsStore();

// Compat: projetos recentes (últimos 10)
export const recentProjectsStore = derived(projectsStore, stryMutAct_9fa48("419") ? () => undefined : (stryCov_9fa48("419"), $s => stryMutAct_9fa48("420") ? $s.list : (stryCov_9fa48("420"), $s.list.slice(0, 10))));