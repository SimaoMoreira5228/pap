<script lang="ts">
  import "../app.css";
  import { toggleMode, setMode, localStorageKey } from "mode-watcher";
  import { Button } from "$lib/components/ui/button";
  import { Sun, Moon, Settings2, BookCopy } from "lucide-svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import { goto } from "$app/navigation";

  if (!localStorage.getItem(localStorageKey)) {
    setMode("light");
  }
</script>

<Toaster />
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
    <div class="flex flex-col justify-between items-center py-6 h-full px-2">
      <div>
        <Button
          variant="outline"
          size="icon"
          on:click={() => goto("/books", { invalidateAll: true })}
        >
          <BookCopy class="w-[1.2rem] h-[1.2rem]" />
        </Button>
      </div>
      <Button
        variant="outline"
        size="icon"
        on:click={() => goto("/settings", { invalidateAll: true })}
      >
        <Settings2 class="w-[1.2rem] h-[1.2rem]" />
      </Button>
    </div>
    <div class="border-l border-muted"></div>
    <div class="flex justify-center items-center w-full h-full p-4">
      <slot />
    </div>
  </div>
</div>
