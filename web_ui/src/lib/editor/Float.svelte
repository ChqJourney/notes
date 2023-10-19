<script>
    import { onDestroy, onMount } from 'svelte';
    import { FloatingMenuPlugin } from '@tiptap/extension-floating-menu';
  
  
    export let editor;
    export let tippyOptions= {};
    export let pluginKey = 'SvelteTiptapFloatingMenu';
    export let shouldShow = null;
    let element;
  
    if (!editor) {
      throw new Error('Missing editor instance.');
    }
  
    onMount(() => {
      const plugin = FloatingMenuPlugin({
        pluginKey,
        editor,
        element,
        tippyOptions,
        shouldShow,
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