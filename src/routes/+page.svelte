<script lang="ts">
 import { invoke } from "@tauri-apps/api/core";
 import { open } from "@tauri-apps/plugin-dialog";
 import { docContent } from "../stores/docStore";
 import { goto } from '$app/navigation';

 async function getFile() {
     try {
         const filepath = await open({
             multiple: false,
             directory: false,
         });

         const response = await invoke('load_file', { filepath: filepath });
         docContent.set(response);
         goto('/edit');

     } catch(error) {
         console.log("Rust error:", error);
     }
 }

 async function newFile() {
    docContent.set('');
    goto('/edit');
 }

</script>

<main class="container">
    <h1>Langnote: Language learning notes</h1>
    <button on:click={newFile}>Create</button>
    <button on:click={getFile}>
        Open file
    </button>
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
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #ffffff;
    background-color: #2f2f2f;
    outline: none;
 }

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

</style>
