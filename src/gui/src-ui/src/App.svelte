<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  interface Template {
    name: String;
    path: String;
    git_path: String;
  }

  let templates: Array<Template> = [];

  function reload_templates() {
    invoke("reload_templates").then((result: Array<Template>) => {
      console.table(result);
      templates = result;
    });
  }
</script>

<main class="container">
  <button on:click={reload_templates}>Reload templates</button>
  {#each templates as template}
    <div>
      <h1>{template[0]}</h1>
      <p>{template[1]}</p>
      <p>{template[2]}</p>
    </div>
  {/each}
</main>

<style>
</style>
