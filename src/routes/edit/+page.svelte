<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import Editor from "../../Editor.svelte";
 // vars
 let title = $state("");
 let subtext = $state("");

 // preload with data:
 let doc = '';

 async function saveTitle(event: Event) {
     event.preventDefault();
     subtext = await invoke("save_title", {title});
 }
 async function docChange(newText: string) {
     doc=newText;
 }
</script>

<main class="container">
    <form onsubmit={saveTitle}>
        <input class="title" placeholder="Title" bind:value={title} />
    </form>
    <div class="editor-pane" onfocus={editorFocus}>
        <Editor doc={doc} onChange={docChange} />
    </div>
</main>

<style>

 .container {
     text-align: left;
     float: left;
     /*padding: 1rem;*/
     /*margin: 0;*/
     padding: 0;
     margin: 0;
     display: flex;
     width: 100vw;
     height: 100vh;

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

 .editor-pane {
     flex: 1;
     border-radius: 0;
     background-color: #2f2f2f;
 }

</style>
