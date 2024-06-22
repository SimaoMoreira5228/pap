import { invoke } from "@tauri-apps/api";
import { toast } from "svelte-sonner";
import { writable } from "svelte/store";

export const DbStringStore = writable(localStorage.getItem("dbUrl") || "");

let DbString: string;

DbStringStore.subscribe(async (value) => {
  try {
    if (value === "") {
      toast.info("A string de conexão da base de dados está vazia");
      return;
    }

    await invoke("init", {
      dbUrl: value,
    });

    toast.success("Connectado com sucesso à base de dados");
  } catch (error) {
    console.error(error);
    toast.error("Erro ao conectar à base de dados");
  }

  localStorage.setItem("dbUrl", value);

  DbString = value;
});

export const DbStringGet = () => {
  return localStorage.getItem("dbUrl") || "";
};

export const DbStringSet = (value: string) => {
  DbStringStore.set(value);
};
