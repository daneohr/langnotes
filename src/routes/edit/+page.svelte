<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 // vars
 let title = $state("");
 let subtext = $state("");

 async function saveTitle(event: Event) {
     event.preventDefault();
     subtext = await invoke("save_title", {title});
 }
</script>

<main class="container">
    <form class="row" onsubmit={saveTitle}>
        <input class="title" placeholder="Title" bind:value={title} />
        <button type="submit">Save</button>
    </form>
    <p>{subtext}</p>
    <p>This is body text!</p>
</main>

<style>
 .container {
     text-align: left;
     float: left;
 }
 .title {
     font-size: 16px;
     background: none;
     border: none;
     box-shadow: none;
     text-align: left;
     float: left;
 }

 form {
     text-align: left;
     float: left;
 }
</style>
