<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import SuccessAlert from "./SuccessAlert.svelte";

  export let selected_template: {
    name: string;
    path: string;
    git_path: string;
  };
  let templates: { name: string; path: string; git_path: string }[] = [];
  let success_alert_shown: boolean = false;
  let success_message: string = String();

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
      });

      if (templates.length == 0) {
        success_message = "No templates found";
      } else {
        success_message = `${templates.length} templates loaded`;
      }

      success_alert_shown = true;
    });
  }

  reload_templates();
</script>

<SuccessAlert {success_message} bind:shown={success_alert_shown} />
<div class="mt-4">
  <button class="btn mx-4" on:click={reload_templates}>Reload templates</button>
</div>
<h2 class="font-bold text-center text-2xl mt-4">Select template:</h2>
<div
  class="grid grid-cols-1 border-4 border-blue-300 mx-4 my-4 px-4 py-4 rounded overflow-y-auto h-[calc(100vh-180px)]"
>
  {#each templates as template}
    <div class="my-2">
      <button
        on:click={() => {
          selected_template = template;
          console.table(selected_template);
        }}
        class="
        btn w-full
        {selected_template.name == template.name ? 'underline' : ''}
        "
      >
        <h1 class="font-bold text-center text-lg">{template.name}</h1>
        <!-- <div class="text-sm text-gray-300">
        <div>
          <p class="font-bold">Path:</p>
          {template.path}
        </div>
        <div>
          <p class="font-bold">URL:</p>
          {template.git_path}
        </div>
      </div> -->
      </button>
    </div>
  {/each}
</div>
