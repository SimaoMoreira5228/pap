<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<ul class={`my-6 ml-6 list-disc [&>li]:mt-2 ${classNames}`}>
  <slot />
</ul>
