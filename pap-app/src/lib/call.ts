import { invoke } from "@tauri-apps/api";
import { dbStringStore, jwtStore } from "./stores";
import { DatabaseConnectionStatus } from "./types";
import { goto } from "$app/navigation";

export async function call<T>(method: string, args: any = {}): Promise<T> {
  let sendSetup = false;
  if (dbStringStore.get().dbUrl === "") goto("/setup");

  if (!args.token) args = { ...args, token: jwtStore.get() };
  while (true) {
    switch (dbStringStore.getProperty<DatabaseConnectionStatus>("status")) {
      case DatabaseConnectionStatus.CONNECTED:
        return await invoke(method, args);
      case DatabaseConnectionStatus.CONNECTING:
        await new Promise((resolve) => setTimeout(resolve, 1000));
        break;
      case DatabaseConnectionStatus.BROKEN:
        if (!sendSetup) {
          goto("/setup");
          sendSetup = true;
        }
        await new Promise((resolve) => setTimeout(resolve, 1000));
      case DatabaseConnectionStatus.NOT_CONNECTED:
        await new Promise((resolve) => setTimeout(resolve, 1000));
      default:
        await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
}
