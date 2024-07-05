import { invoke } from "@tauri-apps/api";
import { toast } from "svelte-sonner";
import { writable, type Writable } from "svelte/store";

class Store<T> {
  private value: T;
  private useLocalStorage = false;
  private name: string;
  private store: Writable<T>;

  constructor(
    value: T,
    useLocalStorage: boolean = false,
    name: string = "",
    callback?: (value: T) => void
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
  }

  set(value: T) {
    this.value = value;
    this.store.set(value);

    if (this.useLocalStorage) {
      localStorage.setItem(this.name, JSON.stringify(value));
    }
  }

  get() {
    return this.value;
  }

  setCallback(callback: (value: T) => void) {
    this.store.subscribe(callback);
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

const redirectStore = new Store(false);

const dbStringStore = new Store(
  safeParseJSON<string>(localStorage.getItem("dbUrl"), ""),
  true,
  "dbUrl",
  async (value) => {
    if (value === "") {
      toast.info("A string de conexão da base de dados está vazia");
      return;
    }

    try {
      await invoke("init", {
        dbUrl: value,
      });

      toast.success("Connectado com sucesso à base de dados");
    } catch (error) {
      toast.error("Erro ao conectar à base de dados");
    }
  }
);

const jwtStore = new Store("");

const loanPeriodStore = new Store(
  safeParseJSON<number>(localStorage.getItem("loanPeriod"), 7),
  true,
  "loanPeriod"
);

export { dbStringStore, jwtStore, loanPeriodStore, redirectStore };
