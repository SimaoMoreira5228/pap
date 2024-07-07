<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Calendar } from "$lib/components/ui/calendar";
  import * as Popover from "$lib/components/ui/popover";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import {
    DateFormatter,
    type DateValue,
    getLocalTimeZone,
    parseDate,
  } from "@internationalized/date";
  import { cn } from "$lib/utils";
  import { CalendarIcon } from "lucide-svelte";
  import type { Autor } from "$lib/types";
  import Icon from "@iconify/svelte";
  import { writable } from "svelte/store";

  let name = "";
  let nationality = "";
  let birthDate: DateValue | undefined = undefined;
  let deathDate: DateValue | undefined = undefined;

  let isLoading = writable(false);

  export let action: "create" | "update";
  export let id: number | undefined = undefined;

  const df = new DateFormatter("pt-PT");

  export let updateAuthors: () => Promise<void> = async () => {};

  async function createAuthor() {
    if (action !== "create") return;

    try {
      if (!name) {
        return toast.error("O nome é obrigatório");
      }

      await call("create_author", {
        name,
        nationality,
        birthDate: birthDate?.toString(),
        deathDate: deathDate?.toString(),
      });

      toast.success("Autor criado com sucesso");

      updateAuthors();
    } catch (error) {
      toast.error(error as string);
    } finally {
      name = "";
      nationality = "";
      birthDate = undefined;
      deathDate = undefined;
    }
  }

  async function updateAuthor() {
    if (action !== "update") return;

    try {
      await call("update_author", {
        id,
        name,
        nationality,
        birthDate: birthDate?.toString(),
        deathDate: deathDate?.toString(),
      });

      toast.success("Autor atualizado com sucesso");

      updateAuthors();
    } catch (error) {
      toast.error(error as string);
    } finally {
      name = "";
      nationality = "";
      birthDate = undefined;
      deathDate = undefined;
    }
  }

  async function handleSubmit() {
    if (action === "create") {
      if (!name) {
        return toast.error("O nome é obrigatório");
      }

      await createAuthor();
    } else if (action === "update") {
      if (!name) {
        return toast.error("O nome é obrigatório");
      }

      await updateAuthor();
    }
  }

  async function getInCaseOfUpdate() {
    if (action === "update") {
      try {
        isLoading.set(true);

        const author = await call<Autor>("get_author_by_id", {
          id,
        });

        name = author.nome;
        nationality = author.nacionalidade ? author.nacionalidade : "";
        birthDate = author.data_nasc ? parseDate(author.data_nasc) : undefined;
        deathDate = author.data_morte
          ? parseDate(author.data_morte)
          : undefined;
      } catch (error) {
        console.error(error);
        toast.error(error as string);
      } finally {
        isLoading.set(false);
      }
    }
  }
</script>

<Dialog.Root>
  <Dialog.Trigger class="cursor-pointer">
    <button on:click={async () => await getInCaseOfUpdate()}>
      <slot name="trigger" />
    </button>
  </Dialog.Trigger>
  {#if $isLoading}
    <div class="flex justify-center items-center w-full h-full">
      <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
    </div>
  {:else}
    <Dialog.Content class="sm:max-w-[425px]">
      <Dialog.Header>
        <Dialog.Title
          >{action === "create"
            ? "Criar Autor"
            : "Atualizar Autor"}</Dialog.Title
        >
      </Dialog.Header>
      <div class="flex flex-col gap-4 py-4">
        <div class="flex flex-col gap-2">
          <Label for="nome">Nome</Label>
          <Input
            type="text"
            id="nome"
            name="nome"
            class="border border-muted rounded-lg"
            bind:value={name}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="nationality">Nacionalidade</Label>
          <Input
            type="text"
            id="nationality"
            name="nationality"
            class="border border-muted rounded-lg"
            bind:value={nationality}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label>Data de nascimento</Label>
          <Popover.Root>
            <Popover.Trigger asChild let:builder>
              <Button
                variant="outline"
                class={cn(
                  "w-[280px] justify-start text-left font-normal",
                  !birthDate && "text-muted-foreground"
                )}
                builders={[builder]}
              >
                <CalendarIcon class="mr-2 h-4 w-4" />
                {birthDate
                  ? df.format(birthDate.toDate(getLocalTimeZone()))
                  : "Escolha uma data"}
              </Button>
            </Popover.Trigger>
            <Popover.Content class="w-auto p-0">
              <Calendar bind:value={birthDate} initialFocus />
            </Popover.Content>
          </Popover.Root>
        </div>
        <div class="flex flex-col gap-2">
          <Label>Data de morte</Label>
          <Popover.Root>
            <Popover.Trigger asChild let:builder>
              <Button
                variant="outline"
                class={cn(
                  "w-[280px] justify-start text-left font-normal",
                  !deathDate && "text-muted-foreground"
                )}
                builders={[builder]}
              >
                <CalendarIcon class="mr-2 h-4 w-4" />
                {deathDate
                  ? df.format(deathDate.toDate(getLocalTimeZone()))
                  : "Escolha uma data"}
              </Button>
            </Popover.Trigger>
            <Popover.Content class="w-auto p-0">
              <Calendar bind:value={deathDate} initialFocus />
            </Popover.Content>
          </Popover.Root>
        </div>
      </div>
      <Dialog.Footer>
        <Dialog.Close>
          <Button type="submit" on:click={handleSubmit}
            >{action === "create" ? "Criar Autor" : "Atualizar Autor"}</Button
          >
        </Dialog.Close>
      </Dialog.Footer>
    </Dialog.Content>
  {/if}
</Dialog.Root>
