/**
 * Typed Tauri IPC Wrapper
 *
 * Provides type-safe access to Tauri commands across all modules.
 * All IPC calls should use this wrapper to ensure type consistency.
 */

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
export async function ipc<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    // Error handling is delegated to the calling code
    // so they can implement context-specific error recovery
    throw error;
  }
}

/**
 * Convenience wrappers for common patterns
 */

export async function ipcQuery<T>(command: string): Promise<T> {
  return ipc<T>(command);
}

export async function ipcMutation<T>(
  command: string,
  args: Record<string, unknown>
): Promise<T> {
  return ipc<T>(command, args);
}

/**
 * Unwraps an ApiResponse<T>, throwing if error is present or data is null.
 * Eliminates the need for non-null assertions on result.data in IPC callers.
 */
export function unwrapResponse<T>(result: { data: T | null; error: string | null }): T {
  if (result.error) throw new Error(result.error);
  if (result.data === null || result.data === undefined) {
    throw new Error('IPC response contained no data and no error');
  }
  return result.data;
}
