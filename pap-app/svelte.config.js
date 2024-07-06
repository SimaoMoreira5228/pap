// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    prerender: {
      entries: [
        "/",
        "/authors",
        "/authors/1",
        "/books/1",
        "/publishers/1",
        "/books",
        "/publishers",
        "/settings",
        "/login",
        "/setup",
      ],
    },
  },
};

export default config;
