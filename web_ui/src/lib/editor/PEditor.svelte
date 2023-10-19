<script>
  import { onMount, onDestroy } from "svelte";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Toolbar from "./Toolbar.svelte";
  import Underline from "@tiptap/extension-underline";
  import BubbleMenu from "@tiptap/extension-bubble-menu";
  import Bubble from "./Bubble.svelte";
  let element;
  let editor;
  let content;
  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [
        StarterKit,
        Underline,
        BubbleMenu.configure({
          element: document.querySelector(".menu"),
        }),
      ],
      editorProps: {
        attributes: {
          class:
            "prose w-full h-96 border rounded-md p-4 dark:prose-invert prose-sm sm:prose-base lg:prose-lg xl:prose-2xl m-5 focus:outline-none",
        },
      },
      content: content,
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor;
      },
    });
  });

  onDestroy(() => {
    if (editor) {
      editor.destroy();
    }
  });
</script>

{#if editor}
  <Bubble editor={editor}>
    <Toolbar
      on:swith_in={(e) => {
        switch (e.detail.type) {
          case "strong":
            editor.chain().focus().toggleBold().run();
            break;
          case "i":
            editor.chain().focus().toggleItalic().run();
            break;
          case "s":
            editor.chain().focus().toggleStrike().run();
            break;
          case "u":
            console.log("u");
            editor.chain().focus().toggleUnderline().run();
            break;
          default:
            break;
        }
      }}
    />
  </Bubble>
{/if}

<div spellcheck="false" bind:this={element} />

<style>
</style>
