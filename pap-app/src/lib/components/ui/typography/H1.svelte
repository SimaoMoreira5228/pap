<script lang="ts">
  let classNames: string = "";
  export { classNames as class };
</script>

<h1
  class={`scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl ${classNames}`}
>
  <slot />
</h1>
