import { invoke } from "@tauri-apps/api/tauri";
import { RustCallResult } from "@/types";

export interface AppConfig {
  model_dir?: string;
  port: number;
  width?: number;
  height?: number;
  x?: number;
  y?: number;
  check_update?: boolean;
  remote_list?: string[];
  model_block?: boolean;
  [key: string]: any;
}

/**
 *读取配置文件
 * @returns
 */
export async function readConfig(): Promise<AppConfig> {
  return invoke<AppConfig>("read_config");
}

/**
 * 写入配置文件
 * @param data 文件内容
 */
export async function writeConfig(data: string) {
  await invoke<AppConfig>("write_config", { value: data });
}

/**
 * 系统文件读取为数组
 * @param path
 * @returns
 */
export async function readSysFileForArray(path: String) {
  const file: RustCallResult<number[]> = await invoke("read_file", {
    filePath: path,
  });
  return file;
}

/**
 * 写入系统文件
 * @param filePath 系统文件目录
 * @param content 文件内容
 */
export async function writeSysFileFromString(
  filePath: string,
  content: string
) {
  const rt = await invoke<RustCallResult<String>>("write_file", {
    filePath,
    data: content,
  });
  return !rt.err;
}

export function sleep(ms: number) {
  return new Promise((fn) => setTimeout(fn, ms));
}

// 节流时间戳版本
export function throttle(func, wait) {
  let previous = 0;
  return function (this) {
    let now = Date.now(),
      context = this,
      args = [...arguments];
    if (now - previous > wait) {
      func.apply(context, args);
      previous = now; // 闭包，记录本次执行时间戳
    }
  };
}
