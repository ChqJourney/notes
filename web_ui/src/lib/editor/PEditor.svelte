<script>
  import { onMount, onDestroy } from "svelte";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Toolbar from "./Toolbar.svelte";
  import Underline from "@tiptap/extension-underline";
  import BubbleMenu from "@tiptap/extension-bubble-menu";
  import Bubble from "./Bubble.svelte";
  import Float from "./Float.svelte";
  import FloatingMenu, { FloatingMenuPlugin } from '@tiptap/extension-floating-menu';
  import Dropcursor from '@tiptap/extension-dropcursor'
import Image from '@tiptap/extension-image'
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
        FloatingMenu.configure({
      element: document.querySelector('.fmenu'),
    }),
        Image,
      Dropcursor
      ],
      editorProps: {
        attributes: {
          class:
            "prose w-full h-96 border rounded-md p-4 dark:prose-invert prose-sm sm:prose-base lg:prose-lg xl:prose-2xl m-5 focus:outline-none",
        },
      },
      content: "",
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor;
      },
      onUpdate:({editor})=>{
        content=editor.getHTML()
      }
    });
  });

  onDestroy(() => {
    if (editor) {
      editor.destroy();
    }
  });
</script>

{#if editor}
  <Bubble {editor}>
    <Toolbar
      {editor}
      on:swith_in={(e) => {
        switch (e.detail.type) {
          case "ful":
            editor.chain().focus().setBulletList().run();
            break;
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
          case "ul":
            editor.chain().focus().toggleBulletList().run();
            break;
          case "ol":
            editor.chain().focus().toggleOrderedList().run();
            break;
          case "h1":
            editor.chain().focus().toggleHeading({ level: 1 }).run();
            break;
          case "h2":
            editor.chain().focus().toggleHeading({ level: 2 }).run();
            break;
          case "h3":
            editor.chain().focus().toggleHeading({ level: 3 }).run();
            break;
          case "h4":
            editor.chain().focus().toggleHeading({ level: 4 }).run();
            break;
          case "h5":
            editor.chain().focus().toggleHeading({ level: 5 }).run();
            break;
          case "h6":
            editor.chain().focus().toggleHeading({ level: 6 }).run();
            break;
          case "quote":
            editor.chain().focus().toggleBlockquote().run();
            break;
          case "code":
            editor.chain().focus().toggleCodeBlock().run();
            break;
          case "image":
            editor.chain().focus().setImage({ src: "vite.svg" }).run();
            break;
          default:
            break;
        }
      }}
    />
  </Bubble>
{/if}

<div spellcheck="false" bind:this={element} />
<button on:click={()=>{
  console.log(editor.getHTML())
  console.log(content)
}}>Output</button>
<style>
</style>
