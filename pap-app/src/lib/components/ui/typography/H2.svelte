<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<h2
  class={`scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0 ${classNames}`}
>
<slot />
</h2>
