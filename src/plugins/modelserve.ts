import { invoke } from "@tauri-apps/api/tauri";
import { readConfig, AppConfig } from "@/util";

export interface Live2dModelItem {
  url: string;
  type: "remote" | "local";
}

export async function model_list(): Promise<Array<Live2dModelItem>> {
  let config = {} as AppConfig;
  let list: Array<Live2dModelItem> = [];
  try {
    config = await readConfig();
    let localList: string[] = [];
    localList = await invoke<Array<string>>("model_list");
    list = list.concat(
      (list = localList.map(
        (it) =>
          ({
            url: it,
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
          url: it,
          type: "remote",
        } as Live2dModelItem)
    )
  );
  return list;
}
