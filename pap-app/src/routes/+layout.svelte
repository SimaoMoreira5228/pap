<script lang="ts">
  import "../app.css";
  import { toggleMode, setMode, localStorageKey } from "mode-watcher";
  import { Button } from "$lib/components/ui/button";
  import { Sun, Moon, Settings2, BookCopy } from "lucide-svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import { afterNavigate, goto } from "$app/navigation";
  import { call } from "$lib/call";
  import type { permissao } from "$lib/types";
  import { toast } from "svelte-sonner";
  import { jwtStore, redirectStore } from "$lib/stores";
  import { writable } from "svelte/store";

  if (!localStorage.getItem(localStorageKey)) {
    setMode("light");
  }

  let settingsPermission = false;
  let showBar = false;
  let refreshBar = writable(false);

  redirectStore.setCallback((value) => {
    refreshBar.set(value);
  });

  refreshBar.subscribe(async (value) => {
    if (value) {
      showBar = false;
      try {
        if (await call<boolean>("check_librarians_existence")) {
          const permissionId = (
            await call<permissao[]>("get_permissions")
          ).find((permission) => permission.acao === "mudar_configuracoes")?.id;

          if (jwtStore.get() !== "") {
            const permissions = await call<permissao[]>(
              "get_librarian_permissions"
            );

            settingsPermission = permissions.some(
              (permission) => permission.id === permissionId
            );
          }
        } else {
          settingsPermission = true;
        }

        showBar = true;
      } catch (error) {
        toast.error(error as string);
        if (error === "Base de dados não inicializada") {
          goto("/setup");
          showBar = false;
        }
      }
    }
  });

  afterNavigate(() => {
    refreshBar.set(true);
  });
</script>

<Toaster class="z-50" />
<Button
  on:click={toggleMode}
  variant="outline"
  size="icon"
  class="absolute right-2 top-2"
>
  <Moon
    class="rotate-0 w-[1.2rem] h-[1.2rem] scale-100 transition-all dark:-rotate-90 dark:scale-0"
  />
  <Sun
    class="absolute w-[1.2rem] h-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
  />
  <span class="sr-only">Mudar o tema</span>
</Button>

<div class="w-screen h-screen flex justify-start">
  <div class="flex flex-row w-full h-full">
    {#if showBar}
      <div class="flex flex-col justify-between items-center py-6 h-full px-2">
        <div>
          <Button variant="outline" size="icon" on:click={() => goto("/books")}>
            <BookCopy class="w-[1.2rem] h-[1.2rem]" />
          </Button>
        </div>
        {#if settingsPermission}
          <Button
            variant="outline"
            size="icon"
            on:click={() => goto("/settings")}
          >
            <Settings2 class="w-[1.2rem] h-[1.2rem]" />
          </Button>
        {/if}
      </div>
      <div class="border-l border-muted"></div>
    {/if}
    <div class="flex justify-center items-center w-full h-full p-4">
      <slot />
    </div>
  </div>
</div>
