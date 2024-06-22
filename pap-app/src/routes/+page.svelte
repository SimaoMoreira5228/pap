<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { H1, P } from "$lib/components/ui/typography/index";
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="flex justify-center items-center flex-col gap-2">
  <H1>Welcome to Tauri!</H1>

  <div>
    <img src="/icon.png" alt="libra hub" />
  </div>

  <P>Click on the Tauri, Vite, and SvelteKit logos to learn more.</P>

  <form class="flex flex-row gap-2" on:submit|preventDefault={greet}>
    <Input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <Button type="submit">Greet</Button>
  </form>

  <P>{greetMsg}</P>
</div>
