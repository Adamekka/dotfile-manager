<script lang="ts">
  import TemplateList from "./lib/TemplateList.svelte";
  import TemplateInfo from "./lib/TemplateInfo.svelte";

  let selected_template: {
    name: string;
    path: string;
    git_path: string;
  } = {
    name: "None",
    path: "None",
    git_path: "None",
  };
</script>

<main class="container">
  <div class="flex">
    <div class="flex-1">
      <TemplateList bind:selected_template />
    </div>
    <div class="flex-1">
      <TemplateInfo bind:selected_template />
    </div>
  </div>
</main>
