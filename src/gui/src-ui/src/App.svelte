<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  interface Template {
    name: String;
    path: String;
    git_path: String;
  }

  function reload_templates() {
    invoke("reload_templates").then((result) => {
      console.log(result);
    });
  }
</script>

<main class="container">
  <button on:click={reload_templates}>Reload templates</button>
</main>

<style>
</style>
