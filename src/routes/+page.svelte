<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import { open } from "@tauri-apps/plugin-dialog";
 import { docContent, docPath } from "../stores/docStore";
 import { goto } from '$app/navigation';

 async function getFile() {
     try {
         // Tauri-specific open command to get files
         const filepath = await open({
             multiple: false,
             directory: false,
         });

         const response = await invoke('load_file', { filepath: filepath });
         docContent.set(response);
         docPath.set(filepath);
         goto('/edit');

     } catch(error) {
         console.log("Rust error:", error);
     }
 }

 async function newFile() {
     docContent.set('');
     docPath.set('');
     goto('/edit');
 }

</script>

<main class="container">
    <h1>Langnote: Language learning notes</h1>
    <button on:click={newFile} class="file-btn">Create</button>
    <button on:click={getFile} class="file-btn">Open file</button>
</main>

<style>
:root {
  font-family: Monospace;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

 .container {
     margin: auto;
     display: flex;
     flex-direction: column;
     justify-content: center;
     text-align: center;
}

 h1 {
     text-align: center;
}

.file-btn {
    align-items: center;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #ffffff;
    background-color: #4f4f4f;
    outline: none;
    border: none;
    margin: 0.5rem;
    /* idk if i like this: */
    /* cursor: pointer; */
 }

.file-btn:hover {
    background-color: #6f6f6f;
 }

.file-btn:active {
    background-color: #e8e8e8;
    color: black;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

</style>
