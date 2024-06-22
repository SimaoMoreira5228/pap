<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { DbStringGet, DbStringSet } from "$lib/stores";

  let dbUrl = DbStringGet();
</script>

<div class="flex justify-start items-start flex-col gap-2 w-full h-full">
  <form class="flex flex-col gap-2 w-full h-full">
    <Label for="dbUrl">Database URL</Label>
    <div class="flex flex-row gap-2 w-full h-full">
      <Input
        id="dbUrl"
        type="text"
        placeholder="Database URL"
        bind:value={dbUrl}
        class="w-[30%]"
      />
      <Button
        type="submit"
        on:click={(event) => {
          event.preventDefault();
          DbStringSet(dbUrl);
        }}
      >
        Save
      </Button>
    </div>
  </form>
</div>
