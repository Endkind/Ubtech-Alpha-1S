import { invoke } from "@tauri-apps/api/core";

/**
 * Returns the directory name of a `path`.
 * If the path has a parent directory, returns it.
 * If the parent is empty or root, returns '.' or the root respectively.
 */
export async function dirname(path: string): Promise<string> {
  return invoke<string>("path_dirname", { path });
}

/**
 * Joins multiple path segments into a single normalized path using native OS separators.
 */
export async function join(...paths: string[]): Promise<string> {
  return invoke<string>("path_join", { paths });
}

/**
 * Returns the last portion of a `path` (file name or last directory name).
 */
export async function basename(path: string): Promise<string> {
  return invoke<string>("path_basename", { path });
}

/**
 * Determines whether `path` is an absolute path.
 */
export async function isAbsolute(path: string): Promise<boolean> {
  return invoke<boolean>("path_is_absolute", { path });
}
