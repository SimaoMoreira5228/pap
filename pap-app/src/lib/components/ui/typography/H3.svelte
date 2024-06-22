<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<h3 class={`scroll-m-20 text-2xl font-semibold tracking-tight ${classNames}`}>
  <slot />
</h3>
