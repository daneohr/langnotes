<!-- EDITOR PAGE  -->
<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import Editor from "../../Editor.svelte";
 import { docContent, docPath } from "../../stores/docStore";
 import { onMount } from "svelte";

 // vars
 let title = $state("");
 let file_extension = $state("");
 let debug_str = $state("");

 // preload with data:
 let doc = $state($docContent);
 let path = $state($docPath);

 onMount(async () => {
    await findTitle(path);
 });

 async function findTitle(path: string) {
     if (!path) return;
     // Compute the 'title' of the file:
     //  example.md turns into 'example' = title
     let final_title = path.slice(path.lastIndexOf("/")+1, path.length);
     let file_ext = final_title.substring(final_title.indexOf(".") + 1);
     final_title = final_title.substring(0, final_title.indexOf("."));

     title = final_title;
     file_extension = file_ext.toUpperCase();
 }

 async function saveTitle(event: Event) {
     event.preventDefault();
     debug_str = await invoke("save_title", {title});
 }

 async function docChange(newText: string) {
     doc = newText;
     docContent.set(newText);
 }

 async function handleSave(event: Event) {
    event.preventDefault();
    // if path == '': ask for path
    debug_str = path;
    await invoke("save_file", { filepath: path, contents: doc });
  }

</script>

<main class="container">
    <div class="header-area">
      <form onsubmit={saveTitle} class="title-form">
          <div class="input-wrapper">
            <input class="title" placeholder="Title" bind:value={title} />
            <span class="file-ext">.{file_extension}</span>
          </div>
      </form>
      {#if debug_str}
          <p class="debug-text">Debug: {debug_str}</p>
      {/if}
      <button onclick={handleSave}>SAVE</button>
    </div>
    <div class="editor-pane" onfocus={editorFocus}>
        <Editor doc={doc} onChange={docChange} />
    </div>
</main>

<style>

 .container {
     text-align: left;
     float: left;
     margin: 0;
     display: flex;
     width: 100vw;
     height: 100vh;
     flex-direction: column;
     box-sizing: border-box;
     gap: 12px;
 }

 .header-area {
     display: flex;
     flex-direction: column;
 }

 .title-form {
     display: flex;
     align-items: baseline;
     gap: 8px;
     margin: 0;
     padding: 0;
     background: transparent;
     height: auto;
 }

 .input-wrapper {
     display: inline-flex;
     align-items: baseline;
     background: transparent;

 }

 .title {
     color: #ffffff;
     font-weight: 600;
     font-size: 1.5rem;
     background: none;
     border: none;
     outline: none;
     box-shadow: none;
     text-align: left;
     padding: 0;
     margin: 0;

     field-sizing: content;
     min-width: 60px;
 }

 .file-ext {
     color: #777777;
     font-size: 0.8rem;
     font-weight: 500;
     user-select: none;
 }

 .debug-text {
     color: #ffff00;
 }

 .editor-pane {
     flex: 1;
     width: 100%;
     border-radius: 0;
     background-color: #2f2f2f;
 }

</style>
