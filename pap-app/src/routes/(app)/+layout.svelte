<script lang="ts">
  import "../../app.css";
  import { Button } from "$lib/components/ui/button";
  import { afterNavigate, goto } from "$app/navigation";
  import { call } from "$lib/call";
  import { toast } from "svelte-sonner";
  import { jwtStore } from "$lib/stores";
  import { writable } from "svelte/store";
  import Icon from "@iconify/svelte";
  import { Sun, Moon } from "lucide-svelte";
  import { Toaster } from "svelte-sonner";
  import { toggleMode, setMode, localStorageKey } from "mode-watcher";
  import { hasPermission } from "$lib/utils";

  if (!localStorage.getItem(localStorageKey)) {
    setMode("light");
  }

  let settingsPermission = false;
  let refreshBar = writable(false);

  afterNavigate(() => {
    refreshBar.set(true);
  });

  $: if ($refreshBar) {
    (async () => {
      try {
        if (await call<boolean>("check_librarians_existence")) {
          if (jwtStore.get() !== "") {
            settingsPermission = await hasPermission("mudar_configuracoes");
          }
        } else {
          settingsPermission = true;
        }
      } catch (error) {
        toast.error(error as string);
      } finally {
        refreshBar.set(false);
      }
    })();
  }
</script>

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

<Toaster class="z-50" />

<div class="flex w-screen h-screen justify-start">
  <div class="flex flex-row w-full h-full">
    <div class="flex flex-col justify-between items-center py-6 h-full px-2">
      <div class="flex flex-col gap-1">
        <Button variant="outline" size="icon" on:click={() => goto("/books")}>
          <Icon
            icon="material-symbols-light:book-4-outline"
            class="w-[1.5rem] h-[1.5rem] text-secondary-foreground"
          />
        </Button>
        <Button variant="outline" size="icon" on:click={() => goto("/authors")}>
          <Icon
            icon="guidance:office"
            class="w-[1.5rem] h-[1.5rem] text-secondary-foreground"
          />
        </Button>
        <Button
          variant="outline"
          size="icon"
          on:click={() => goto("/publishers")}
        >
          <Icon
            icon="ph:factory-thin"
            class="w-[1.5rem] h-[1.5rem] text-secondary-foreground"
          />
        </Button>
        <Button
          variant="outline"
          size="icon"
          on:click={() => goto("/readers")}
        >
          <Icon
            icon="ph:book-open-user-light"
            class="w-[1.5rem] h-[1.5rem] text-secondary-foreground"
          />
        </Button>
      </div>
      {#if settingsPermission}
        <Button
          variant="outline"
          size="icon"
          on:click={() => goto("/settings")}
        >
          <Icon
            icon="carbon:settings-adjust"
            class="w-[1.5rem] h-[1.5rem] text-secondary-foreground"
          />
        </Button>
      {/if}
    </div>

    <div class="border-l border-muted"></div>

    <div class="flex justify-center items-center w-full h-full p-4">
      <slot />
    </div>
  </div>
</div>
