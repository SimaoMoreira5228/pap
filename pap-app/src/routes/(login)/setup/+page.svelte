<script lang="ts">
  import { goto } from "$app/navigation";
  import * as Card from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Switch } from "$lib/components/ui/switch";
  import { dbStringStore } from "$lib/stores";
  import Icon from "@iconify/svelte";
  import { writable } from "svelte/store";

  let dbUrl = "";
  let makeTables = false;
  $: isLoading = writable(false);
</script>

<div class="flex items-center justify-center h-screen w-screen p-8">
  <Card.Root class="shadow-lg">
    <Card.Header>
      <Card.Title> Configurações da base de dados </Card.Title>
      <Card.Description></Card.Description>
    </Card.Header>
    <Card.Content class="mt-4">
      <form class="flex flex-col gap-2">
        <Label for="dbUrl">URL da base de dados</Label>
        <div class="flex flex-row gap-2 w-full justify-start items-center">
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
                dbStringStore.set({dbUrl, makeTables});
                setTimeout(() => {
                  isLoading.set(false);
                  goto("/login");
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
          <div class="flex flex-row gap-2 justify-start items-center">
              <p>Criar tabelas</p>
              <Switch bind:checked={makeTables} />
          </div>
      </form>
    </Card.Content>
  </Card.Root>
</div>
