<script lang="ts">
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Search } from "lucide-svelte";

  let search: string | null;
  export let searchFunction: (value: string | null) => void;
  let className: string | undefined = undefined;

  export { className as class };
</script>

<form
  class={"flex flex-row gap-2 " + className}
  on:submit|preventDefault={() => searchFunction(search)}
>
  <Input bind:value={search} placeholder="Pesquisar..." />
  <Button type="submit" variant="outline" size="icon">
    <Search class="w-[1.2rem] h-[1.2rem]" />
  </Button>
</form>
