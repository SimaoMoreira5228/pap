<script lang="ts">
  import { call } from "$lib/call";
  import type { Categoria, SubCategoria } from "$lib/types";
  import * as Accordion from "$lib/components/ui/accordion";
  import { Button } from "$lib/components/ui/button";
  import { hasPermission } from "$lib/utils";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { toast } from "svelte-sonner";
  import NewCategoryDialog from "$lib/components/custom/NewCategoryDialog.svelte";
  import NewSubCategoryDialog from "$lib/components/custom/NewSubCategoryDialog.svelte";
  import { H3 } from "$lib/components/ui/typography";

  let categories: Categoria[] = [];
  let subCategories: SubCategoria[] = [];

  let hasCreateCategoryPermission = false;
  let hasDeleteCategoryPermission = false;
  let hasCreateSubCategoryPermission = false;
  let hasDeleteSubCategoryPermission = false;
  let isCategoryDeleteLoading = writable(false);
  let isSubCategoryDeleteLoading = writable(false);

  async function loadCategories() {
    try {
      categories = await call("get_categories");
      subCategories = await call("get_sub_categories");

      hasDeleteCategoryPermission = await hasPermission("apagar_categoria");
      hasDeleteSubCategoryPermission = await hasPermission(
        "apagar_sub_categoria"
      );
      hasCreateSubCategoryPermission = await hasPermission(
        "criar_sub_categoria"
      );
      hasCreateCategoryPermission = await hasPermission("criar_categoria");
    } catch (error) {
      console.error(error);
    }
  }

  onMount(async () => {
    await loadCategories();
  });

  async function deleteCategory(category: Categoria) {
    try {
      isCategoryDeleteLoading.set(true);

      await call("delete_category", {
        id: category.id,
      });

      toast.success("Categoria excluída com sucesso");

      await loadCategories();
    } catch (error) {
      toast.error(error as string);
    } finally {
      isCategoryDeleteLoading.set(false);
    }
  }

  async function deleteSubCategory(subCategory: SubCategoria) {
    try {
      isSubCategoryDeleteLoading.set(true);

      await call("delete_sub_category", {
        id: subCategory.id,
      });

      toast.success("Subcategoria excluída com sucesso");

      await loadCategories();
    } catch (error) {
      toast.error(error as string);
    } finally {
      isSubCategoryDeleteLoading.set(false);
    }
  }
</script>

<div class="flex flex-col gap-2 overflow-auto w-full h-full">
  <H3>Categorias & Subcategorias</H3>
  <Accordion.Root class="w-full overflow-auto">
    {#each categories as category}
      <Accordion.Item
        value={category.id.toString()}
        class="rounded-lg border p-2"
      >
        <Accordion.Trigger>
          <div class="flex flex-row items-center justify-between w-[97%]">
            <p class="text-lg font-bold">{category.nome}</p>
            {#if hasDeleteCategoryPermission}
              {#if $isCategoryDeleteLoading}
                <div class="flex justify-center items-center w-full h-full">
                  <Icon
                    icon="svg-spinners:270-ring-with-bg"
                    class="w-6 h-6 text-primary"
                  />
                </div>
              {:else}
                <Button
                  variant="destructive"
                  size="icon"
                  on:click={() => {
                    deleteCategory(category);
                  }}
                >
                  <Icon
                    icon="material-symbols-light:delete-outline-rounded"
                    class="w-6 h-6 text-secondary-muted"
                  />
                </Button>
              {/if}
            {/if}
          </div>
        </Accordion.Trigger>
        <Accordion.Content>
          <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
            {#each subCategories.filter((subCategory) => subCategory.id_categoria === category.id) as subCategory}
              <div
                class="flex flex-row rounded-lg border p-2 items-center justify-between w-full"
              >
                <p>{subCategory.nome}</p>
                {#if hasDeleteSubCategoryPermission}
                  {#if $isSubCategoryDeleteLoading}
                    <div class="flex justify-center items-center w-full h-full">
                      <Icon
                        icon="svg-spinners:270-ring-with-bg"
                        class="w-8 h-8 text-primary"
                      />
                    </div>
                  {:else}
                    <Button
                      variant="destructive"
                      size="icon"
                      on:click={() => {
                        deleteSubCategory(subCategory);
                      }}
                    >
                      <Icon
                        icon="material-symbols-light:delete-outline-rounded"
                        class="w-6 h-6 text-secondary-muted"
                      />
                    </Button>
                  {/if}
                {/if}
              </div>
            {/each}
            {#if hasCreateSubCategoryPermission}
              <NewSubCategoryDialog
                updateCategories={loadCategories}
                categoryId={category.id}
              >
                <Button slot="trigger" class="w-full"
                  >Adicionar nova sub-categoria</Button
                >
              </NewSubCategoryDialog>
            {/if}
          </div>
        </Accordion.Content>
      </Accordion.Item>
    {/each}
  </Accordion.Root>

  {#if hasCreateCategoryPermission}
    <NewCategoryDialog updateCategories={loadCategories}>
      <Button slot="trigger">Adicionar nova categoria</Button>
    </NewCategoryDialog>
  {/if}
</div>
