<script lang="ts">
 import { onMount, onDestroy } from "svelte";
 import { EditorState } from "@codemirror/state";
 import { EditorView, basicSetup } from "codemirror";
 import { vim } from "@replit/codemirror-vim";

 // depricated but what else is there tbh..:
 import { markdown } from "@codemirror/lang-markdown";


 export let doc: string = '';
 export let onChange: (text: string) => void = () => {};

 let element = HTMLDivElement;
 let view = EditorView;
 onMount (() => {
     const editorTheme = EditorView.theme({
         "&": {
             color: "#ffffff",
             background: "#2f2f2f",
             height: "100%"
         },

         ".cm-content": {
             caretColor: "#4f4f4f",
         },

         "&.cm-focused .cm-cursor": {
             borderLeftColor: "#afafaf"
         },

         ".cm-gutters": {
             backgroundColor: "#2f2f2f",
             //border: "2px 0 0 0 solid #2f2f2f"
             border: "none"
         },

         ".cm-activeLine, .cm-activeLineGutter": {
             backgroundColor: "#3f3f3f",
         },

         // normal-mode cursor when editor is focused
         "&.cm-focused .cm-fat-cursor": {
             background: "#afafaf !important",
             outline: "1px solid #afafaf !important"
        },

         // normal-mode cursor when editor is not focused
         ".cm-fat-cursor": {
             outline: "1px solid #8f8f8f !important",
             background: "transparent !important"
         },

         // visual mode selection when focused
         "&.cm-focused .cm-selectionBackground, ::selection": {
             // backgroundColor: "#333333 !important"
             backgroundColor: "#ff0000 !important"
         },

         //visual mode selection when not focused
         ".cm-selectionBackground": {
             //backgroundColor: "#222222 !important"
             backgroundColor: "#00ff00 !important"
         },

         ".cm-vim-panel": {
             padding: "0",
             backgroundColor: "#3f3f3f",
             color: "#ffffff",
             fontSize: "13px",
             borderTop: "1px solid #afafaf"
         }

     }, { dark: true });

     const startState = EditorState.create({
         doc: doc,
         extensions: [
             basicSetup,
             vim(),
             markdown(),
             editorTheme,
             EditorView.updateListener.of((update) => {
                 if (update.docChanged) {
                     const currentText = update.state.doc.toString();
                     onChange(currentText);
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
  
 .editor-container  {
     height: 100%;
  }
</style>
