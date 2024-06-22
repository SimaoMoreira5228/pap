<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<p class={`leading-7 [&:not(:first-child)]:mt-6 ${classNames}`}>
  <slot />
</p>
