<script lang="ts">
  import "../app.css";
  import * as Resizable from "$lib/components/ui/resizable";
  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button";
  import { Sun, Moon } from "lucide-svelte";
  import { Toaster } from "$lib/components/ui/sonner";
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
  <span class="sr-only">Toggle theme</span>
</Button>

<div class="w-screen h-screen flex justify-start items-center">
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane maxSize={10} minSize={3} defaultSize={3}></Resizable.Pane>
    <Resizable.Handle />
    <Resizable.Pane>
      <div class="flex justify-center items-center w-full h-full">
        <slot />
      </div>
    </Resizable.Pane>
  </Resizable.PaneGroup>
</div>
