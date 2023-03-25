<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import SuccessAlert from "./SuccessAlert.svelte";

  let templates: { name: string; path: string; git_path: string }[] = [];
  let success_alert_shown: boolean = false;

  function reload_templates() {
    invoke("reload_templates").then((result: Array<Array<string>>) => {
      // Clear the array before adding new items to it to avoid duplicates
      templates = [];
      console.table(result);
      // Add each template to the array of objects
      result.forEach(function (template) {
        let name = template[0];
        let path = template[1];
        let git_path = template[2];
        templates.push({ name, path, git_path });
        success_alert_shown = true;
      });
    });
  }

  reload_templates();
</script>

<SuccessAlert
  success_message="Templates reloaded"
  bind:shown={success_alert_shown}
/>
<button class="btn"on:click={reload_templates}>Reload templates</button>
{#each templates as template}
  <li>
    <h1>{template.name}</h1>
    <p>{template.path}</p>
    <p>{template.git_path}</p>
  </li>
{/each}
