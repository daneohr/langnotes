<!-- EDITOR PAGE  -->
<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import Editor from "../../Editor.svelte";
 import { docContent, docPath } from "../../stores/docStore";
 import { onMount } from "svelte";
 import { goto } from "$app/navigation";
 import { open } from "@tauri-apps/plugin-dialog";

 // vars
 let title = $state("");
 let file_extension = $state("");
 let debug_str = $state("");

 // info about certain errors, etc
 let info_str = $state("");
 let info_str_color = $state("#ffffff");

 // preload with data:
 let doc = $state($docContent);
 let path = $state($docPath);

 let originalTitle = $state("");
 let hasEditedTitle = false;

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
     file_extension = "." + file_ext.toUpperCase();
 }

 async function saveTitle(event: Event) {
     event.preventDefault();
     hasEditedTitle = true;
 }

 async function docChange(newText: string) {
     doc = newText;
     docContent.set(newText);
 }

 // I would love to use an enum for msg_type..
 // ... But they aren't supported in svelte <script>
 function handleInfoStr(msg_type: string, message: string) {
     // TODO: Handle close button

     // Yes, this sucks
     switch (msg_type.toLowerCase().trim()) {
         case "info":
             info_str = "INFO: " + message;
             info_str_color = "#ffff00";
             break;

         case "error":
             info_str = "ERROR: " + message;
             info_str_color = "#ff2222";
             break;

         default:
             throw new Error("Unnown command type");
             break;
     }
 }

 async function handleSave() {

     // TODO: Handle rename files by checking if title has changed
     // I'd need to track how the title came in originally probably
     try {
         if (!$docPath || hasEditedTitle) {
             if (!title) {
                 handleInfoStr("error", "No Title");
                 return;
             }
             else if (title.indexOf(".") > -1 ) {
                 handleInfoStr("error", "Title cannot have '.' in it");
                 return;
             }
             else {
                 const base_filepath = await open({
                     multiple: false,
                     directory: true
                 });
                 if (!base_filepath) {
                     handleInfoStr("info", "Dialog cancelled");
                 }
                 let filepath = await invoke("get_new_filepath", {
                     title: title,
                     basePath: base_filepath
                 });
                     // error, exists
                     // May iterate, so add /path/filename.md turns into /path/filename(1).md, etc
                     handleInfoStr("error", "File already exists");

                 } else {
                     docPath.set(filepath);
                     originalTitle = title;
                 }
             }
         }

         if (!$docPath) {
             handleInfoStr("error", "Invalid File Path");
             return;
         }

         let result = await invoke("save_file", {
             filepath: $docPath,
             contents: doc
         });

         handleInfoStr("info", result);
         hasEditedTitle = false;
     } catch (e) {
         console.error("error: ", e);
         handleInfoStr("error", String(e));
     }
 }

 async function handleQuit() {
     docContent.set('');
     docPath.set('');

     goto('/');
 }

</script>

<main class="container">
    <div class="header-area">
      <form onkeyup={saveTitle} class="title-form">
          <div class="input-wrapper">
            <input class="title" placeholder="Title" bind:value={title} />
            <span class="file-ext">{file_extension}</span>
          </div>
      </form>
      {#if info_str}
          <p class="info-text" style="color: {info_str_color}">{info_str}</p>
      {/if}

      {#if debug_str}
          <p class="debug-text">Debug: {debug_str}</p>
      {/if}
      <button onclick={handleSave}>SAVE</button>
    </div>
    <div class="editor-pane" onfocus={editorFocus}>
        <Editor doc={doc} onChange={docChange} onSave={handleSave} onQuit={handleQuit} />
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

 .debug-text, .info-text{
     color: #ffff00;
 }

 .editor-pane {
     flex: 1;
     width: 100%;
     border-radius: 0;
     background-color: #2f2f2f;
 }

</style>
