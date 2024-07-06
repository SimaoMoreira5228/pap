<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { dbStringStore } from "$lib/stores";
  import Icon from "@iconify/svelte";
  import { writable } from "svelte/store";

  let dbUrl = "";
  $: isLoading = writable(false);
</script>

<div class="flex items-center justify-center h-screen w-screen p-8">
  <form class="flex flex-col gap-4">
    <Label for="dbUrl">URL da base de dados</Label>
    <div class="flex flex-row gap-2">
      <Input
        id="dbUrl"
        type="text"
        placeholder="mysql://user:password@localhost/libra_hub"
        bind:value={dbUrl}
        class="w-[70%]"
      />
      {#if !$isLoading}
        <Button
          type="submit"
          on:click={(event) => {
            event.preventDefault();
            isLoading.set(true);
            dbStringStore.set(dbUrl);
            setTimeout(() => {
              isLoading.set(false);
              goto("/");
            }, 1000);
          }}
        >
          Guardar
        </Button>
      {:else}
        <Icon
          icon="svg-spinners:270-ring-with-bg"
          class="w-8 h-8 text-primary"
        />
      {/if}
    </div>
  </form>
</div>
