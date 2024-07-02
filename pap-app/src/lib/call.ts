import { invoke } from "@tauri-apps/api";
import { jwtStore } from "./stores";

export async function call<T>(method: string, args: any = {}): Promise<T> {
  if (!args.token) args = { ...args, token: jwtStore.get() };
  return await invoke(method, args);
}
