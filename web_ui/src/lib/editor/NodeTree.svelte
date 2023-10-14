<script>
    import Node from "./Node.svelte";
    import { onMount } from "svelte";

 export let tree
 let area
let observer
let editable=true
  onMount(() => {
        observer = new MutationObserver((records, ob) => {
            if(records[0].addedNodes.length>0){

            }
            console.log(records)
            // sourceData=area.innerHTML;
        });
        if (area) {
            observer.observe(area, {
                
                childList:true,
                characterData:true,
                subtree: true,
            });
        }
    });
</script>



<div bind:this={area} spellcheck="false" contenteditable={editable} class="w-full inline-block h-96 rounded-md border-blue-600">
    
    <!-- <Node styleAttr={tree.styleAttr} nodeName={tree.nodeName} nodeType={tree.nodeType} childNodes={tree.childNodes} textContent={tree.textContent}/> -->
</div>

