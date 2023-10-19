<script>
    import { onDestroy, onMount } from 'svelte';
    import { BubbleMenuPlugin } from '@tiptap/extension-bubble-menu';
  
  
    export let editor;
    export let tippyOptions = {};
    export let pluginKey = 'SvelteTiptapBubbleMenu';
    export let shouldShow = null;
    export let updateDelay = 250;
    let element;
  
    if (!editor) {
      throw new Error('Missing editor instance.');
    }
  
    onMount(() => {
      const plugin = BubbleMenuPlugin({
        pluginKey,
        editor,
        element,
        tippyOptions,
        shouldShow,
        updateDelay,
      });
  
      editor.registerPlugin(plugin);
    });
  
    onDestroy(() => {
      editor.unregisterPlugin(pluginKey);
    });
  </script>
  
  <div bind:this={element} class={$$props.class} style="visibility: hidden;">
    <slot />
  </div>