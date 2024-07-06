import { invoke } from "@tauri-apps/api";
import { toast } from "svelte-sonner";
import { writable, type Writable } from "svelte/store";
import { DatabaseConnectionStatus } from "./types";

export class Store<T> {
  private value: T;
  private useLocalStorage = false;
  private name: string;
  private store: Writable<T>;
  private properties: Record<string, any>;

  constructor(
    value: T,
    useLocalStorage: boolean = false,
    name: string = "",
    callback?: (value: T) => void,
    properties: Record<string, any> = {}
  ) {
    this.value = value;
    this.useLocalStorage = useLocalStorage;
    this.name = name;

    this.store = writable(this.value);

    if (this.useLocalStorage) {
      localStorage.setItem(this.name, JSON.stringify(value));
    }

    if (callback) {
      this.store.subscribe(callback);
    }

    this.properties = properties;
  }

  set(value: T) {
    this.value = value;
    this.store.set(value);

    if (this.useLocalStorage) {
      localStorage.setItem(this.name, JSON.stringify(value));
    }

    return this.value;
  }

  get() {
    return this.value;
  }

  setCallback(callback: (value: T) => void) {
    this.store.subscribe(callback);
  }

  setProperty(property: string, value: any) {
    this.properties[property] = value;
  }

  getProperty<T>(property: string): T {
    return this.properties[property];
  }
}

function safeParseJSON<T>(json: string | null, fallback: T): T {
  if (json === null) return fallback;
  try {
    return JSON.parse(json);
  } catch {
    return fallback;
  }
}

const dbStringStore = new Store(
  safeParseJSON<string>(localStorage.getItem("dbUrl"), ""),
  true,
  "dbUrl",
  () => {},
  {
    status: DatabaseConnectionStatus.NOT_CONNECTED,
  }
);

dbStringStore.setCallback(async (value) => {
  if (value === "") {
    toast.info("A string de conexão da base de dados está vazia");
    return;
  }

  dbStringStore.setProperty("status", DatabaseConnectionStatus.CONNECTING);

  try {
    await invoke("init", {
      dbUrl: value,
    });

    toast.success("Connectado com sucesso à base de dados");
    dbStringStore.setProperty("status", DatabaseConnectionStatus.CONNECTED);
  } catch (error) {
    toast.error("Erro ao conectar à base de dados");
    dbStringStore.setProperty("status", DatabaseConnectionStatus.BROKEN);
  }
});

const jwtStore = new Store("");

const loanPeriodStore = new Store(
  safeParseJSON<number>(localStorage.getItem("loanPeriod"), 7),
  true,
  "loanPeriod"
);

export { dbStringStore, jwtStore, loanPeriodStore };
