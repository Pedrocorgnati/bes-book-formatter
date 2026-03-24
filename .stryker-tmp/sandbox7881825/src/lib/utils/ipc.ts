/**
 * Typed Tauri IPC Wrapper
 *
 * Provides type-safe access to Tauri commands across all modules.
 * All IPC calls should use this wrapper to ensure type consistency.
 */
// @ts-nocheck
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
import { invoke } from '@tauri-apps/api/core';

/**
 * Generic type-safe wrapper for Tauri IPC commands
 *
 * @template T - The return type of the command
 * @param command - The Tauri command name
 * @param args - Optional arguments to pass to the command
 * @returns Promise resolving to the command result
 *
 * @example
 * ```typescript
 * // Simple query
 * const projects = await ipc<Project[]>('get_projects');
 *
 * // With arguments
 * const result = await ipc<boolean>('delete_project', { id: '123' });
 * ```
 */
export async function ipc<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (stryMutAct_9fa48("565")) {
    {}
  } else {
    stryCov_9fa48("565");
    try {
      if (stryMutAct_9fa48("566")) {
        {}
      } else {
        stryCov_9fa48("566");
        return await invoke<T>(command, args);
      }
    } catch (error) {
      if (stryMutAct_9fa48("567")) {
        {}
      } else {
        stryCov_9fa48("567");
        // Error handling is delegated to the calling code
        // so they can implement context-specific error recovery
        throw error;
      }
    }
  }
}

/**
 * Convenience wrappers for common patterns
 */

export async function ipcQuery<T>(command: string): Promise<T> {
  if (stryMutAct_9fa48("568")) {
    {}
  } else {
    stryCov_9fa48("568");
    return ipc<T>(command);
  }
}
export async function ipcMutation<T>(command: string, args: Record<string, unknown>): Promise<T> {
  if (stryMutAct_9fa48("569")) {
    {}
  } else {
    stryCov_9fa48("569");
    return ipc<T>(command, args);
  }
}