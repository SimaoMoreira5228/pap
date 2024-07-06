import { invoke } from "@tauri-apps/api";
import { dbStringStore, jwtStore } from "./stores";
import { DatabaseConnectionStatus } from "./types";

export async function call<T>(method: string, args: any = {}): Promise<T> {
  if (!args.token) args = { ...args, token: jwtStore.get() };
  while (true) {
    switch (dbStringStore.getProperty<DatabaseConnectionStatus>("status")) {
      case DatabaseConnectionStatus.CONNECTED:
        return await invoke(method, args);
      case DatabaseConnectionStatus.CONNECTING:
        await new Promise((resolve) => setTimeout(resolve, 1000));
        break;
      case DatabaseConnectionStatus.BROKEN:
        throw new Error("A conexão com a base de dados foi perdida");
      default:
        throw new Error("A conexão com a base de dados não foi estabelecida");
    }
  }
}
