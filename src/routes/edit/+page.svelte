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
    <form onsubmit={saveTitle}>
        <input class="title" placeholder="Title" bind:value={title} />
    </form>
    <textarea class="body-text" type="text" placeholder="Start typing..."></textarea>
</main>

<style>
 .container {
     text-align: left;
     float: left;
     padding: 1rem;
     margin: 0;

 }
 .body-text {
     background: none;
     font-size: 14px;
     background: none;
     border: none;
     box-shadow: none;
     text-align: left;
     float: left;
     padding: 0;
     padding-bottom: 0.5rem;
     margin: 0;
 }
 .title {
     font-size: 18px;
     background: none;
     border: none;
     box-shadow: none;
     text-align: left;
     float: left;
     padding: 0;
     padding-bottom: 0.5rem;
     margin: 0;
 }

 form {
     text-align: left;
     float: left;
     margin: 0;
     padding: 0;
 }

</style>
