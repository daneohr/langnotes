<!-- EDITOR PAGE  -->
<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import Editor from "../../Editor.svelte";
 import { docContent, docPath } from "../../stores/docStore";
 import { onDestroy, onMount } from "svelte";

 // vars
 let title = $state("");
 let file_extension = $state("");
 let subtext = $state("");

 // preload with data:
 let doc = $docContent;
 let path = $docPath;

 onMount(async () => {
    await findTitle(path);
 });

 async function findTitle(path: string) {
     // FIND THE NAME OF THE FILE
     let final_title = path.slice(path.lastIndexOf("/")+1, path.length);
     let file_ext = final_title.substring(final_title.indexOf(".") + 1);
     final_title = final_title.substring(0, final_title.indexOf("."));

     title = final_title;
     file_extension = file_ext;
 }

 async function save(path: string) {
     // if path == '': ask for path
 }

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
        <span>.{file_extension}</span>
        <p>{subtext}</p>
    </form>
    <div class="editor-pane" onfocus={editorFocus}>
        <Editor doc={doc} onChange={docChange} />
    </div>
</main>

<style>

 .container {
     text-align: left;
     float: left;
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
     color: #ffffff;
     font-weight: 600;
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
