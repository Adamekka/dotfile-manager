<script lang="ts">
  export let selected_template: {
    name: string;
    path: string;
    git_path: string;
  } = {
    name: "None",
    path: "None",
    git_path: "None",
  };
</script>

<h2 class="font-bold text-center text-2xl">
  Selected template: {selected_template.name}
</h2>
<!-- <button class="btn">Pull</button>
<button class="btn">Push</button> -->
