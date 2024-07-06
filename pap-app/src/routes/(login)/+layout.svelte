<script>
  import "../../app.css";
  import { Sun, Moon } from "lucide-svelte";
  import { Toaster } from "svelte-sonner";
  import { toggleMode, setMode, localStorageKey } from "mode-watcher";
  import { Button } from "$lib/components/ui/button";

  if (!localStorage.getItem(localStorageKey)) {
    setMode("light");
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
<div class="w-screen h-screen flex justify-start">
  <slot />
</div>
