<script>
    import { onMount, onDestroy } from 'svelte'
    import { Editor } from '@tiptap/core'
    import StarterKit from '@tiptap/starter-kit'
    import Toolbar from './Toolbar.svelte';
  
    let element
    let editor
  
    onMount(() => {
      editor = new Editor({
        element: element,
        extensions: [
          StarterKit,
        ],
        editorProps: {
    attributes: {
      class: 'prose w-full h-96 border rounded-md p-4 dark:prose-invert prose-sm sm:prose-base lg:prose-lg xl:prose-2xl m-5 focus:outline-none',
    },
  },
        content: '',
        onTransaction: () => {
          // force re-render so `editor.isActive` works as expected
          editor = editor
        },
      })
    })
  
    onDestroy(() => {
      if (editor) {
        editor.destroy()
      }
    })
  </script>

{#if editor}
<Toolbar on:swith_in={e=>{
    switch(e.detail.type){
        case "strong":
            editor.chain().focus().toggleBold().run();
            break;
            case "":
            // console.log()
            break;
            default:
                break;
    }
}}/>
{/if}

<div bind:this={element} />

<style>
</style>