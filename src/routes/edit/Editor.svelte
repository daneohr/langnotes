<script lang="ts">
 import { onMount, onDestroy } from "svelte";
 import { EditorState } from "@codemirror/state";
 import { EditorView, basicSetup } from "codemirror";
 import { vim } from "@replit/codemirror-vim";

 export let doc: String = '';
 export let onChange: (text: string) => void = () => {};

 let element = HTMLDivElement;
 let view = EditorView;


 onMount (() => {
     const startState = EditorState.create({
         doc: doc,
         extensions: [
             basicSetup,
             vim(),
             EditorView.updateListener.of((update) => {
                 if (update.docChanged) {
                     const currentText = update.state.doc.toString();
                     onchange(currentText);
                 }
             }),
         ],
     });

     view = new EditorView({
         state: startState,
         parent: element,
     });
 });

 onDestroy(() => {
     if(view) {
         view.destroy();
     }
 });
</script>

<div bind:this={element} class="editor-container"></div>

<style>
  .editor-container {
    width: 100%;
    height: 100%;
    text-align: left;
    border-radius: 4px;
    overflow: hidden;
  }
  
  .editor-container :global(.cm-editor) {
    height: 100%;
    background-color: #1e1e1e;
    color: #d4d4d4;
  }
</style>
