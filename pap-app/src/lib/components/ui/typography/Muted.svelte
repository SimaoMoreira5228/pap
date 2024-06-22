<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<p class={`text-sm text-muted-foreground ${classNames}`}>
  <slot />
</p>
