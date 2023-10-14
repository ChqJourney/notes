<script>
    export let nodeName, nodeType, styleAttr="", childNodes=[];
    export let textContent=""
    console.log(childNodes)
</script>

{#if nodeType===3}
    {textContent}
    {#each childNodes as node}
        <svelte:self nodeName={node.nodeName} nodeType={node.nodeType} style={node.styleAttr} textContent={node.textContent}></svelte:self>
    {/each}
{:else}
<svelte:element this={nodeName} style={styleAttr}>
    {textContent}
    {#each childNodes as node}
        <svelte:self nodeName={node.nodeName} nodeType={node.nodeType} style={node.styleAttr} textContent={node.textContent}></svelte:self>
    {/each}
</svelte:element>
{/if}


