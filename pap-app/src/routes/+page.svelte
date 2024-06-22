<script lang="ts">
  import { env } from "$env/dynamic/private";
  import { H1, P } from "$lib/components/ui/typography/index";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";

  let name = "";
  let greetMsg = "";

  onMount(async () => {
    try {
      await invoke("init", {
        dbUrl: env.DB_CONNECTION,
      });
    } catch (error) {
      console.error(error);
      toast.error("Failed to connect to the database");
    }
  });
</script>

<div class="flex justify-center items-center flex-col gap-2">
  <H1>Welcome to Tauri!</H1>

  <div>
    <img src="/icon.png" alt="libra hub" />
  </div>

  <P>Click on the Tauri, Vite, and SvelteKit logos to learn more.</P>

  <P>{greetMsg}</P>
</div>
