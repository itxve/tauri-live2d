import { invoke } from "@tauri-apps/api/tauri";
import { readConfig } from "@/util";

export interface Live2dModelItem {
  url: string;
  type: "remote" | "local";
}

export async function start(): Promise<number> {
  return await invoke("plugin:model|start_serve");
}

export async function model_list(): Promise<Array<Live2dModelItem>> {
  const config = await readConfig();
  let list: Array<Live2dModelItem> = [];
  try {
    const localList = await invoke<Array<string>>("plugin:model|model_list", {
      serveDir: config.serve_path,
    });
    list = list.concat(
      (list = localList.map(
        (it) =>
          ({
            url: it.replace(config.serve_path!, "http://localhost:3004"),
            type: "local",
          } as Live2dModelItem)
      ))
    );
  } catch (error) {
    list = [];
  }

  const remote_list = config.remote_list || [];

  list = list.concat(
    remote_list.map(
      (it) =>
        ({
          url: it.replace(config.serve_path!, ""),
          type: "remote",
        } as Live2dModelItem)
    )
  );
  return list;
}

export async function shutdown(): Promise<number> {
  return await invoke("plugin:model|shutdown_cmd");
}
