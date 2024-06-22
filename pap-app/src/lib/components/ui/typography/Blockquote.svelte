<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<blockquote class={`mt-6 border-l-2 pl-6 italic ${classNames}`}>
  <slot />
</blockquote>
