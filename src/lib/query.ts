import { type Item } from '~/types';
import { invoke } from "@tauri-apps/api/tauri";

export function getFolders() {
  return invoke<string[]>("get_folders");
}

export function getPasswords(folder: string) {
  return invoke<Item[]>("get_passwords", { folder });
}

