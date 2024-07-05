<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { dbStringStore } from "$lib/stores";

  let dbUrl = "";
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
      <Button
        type="submit"
        on:click={(event) => {
          event.preventDefault();
          dbStringStore.set(dbUrl);
          setTimeout(() => goto("/"), 1000);
        }}
      >
        Guardar
      </Button>
    </div>
  </form>
</div>
