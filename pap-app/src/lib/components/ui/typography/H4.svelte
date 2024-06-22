<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<h4 class={`scroll-m-20 text-xl font-semibold tracking-tight ${classNames}`}>
  <slot />
</h4>
