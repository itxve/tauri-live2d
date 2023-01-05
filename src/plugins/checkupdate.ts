import { invoke } from "@tauri-apps/api/tauri";

export async function check_version_update(): Promise<void> {
  return await invoke("plugin:checkupdate|run_check_update");
}
