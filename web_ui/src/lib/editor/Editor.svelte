<script>
    import { onMount } from "svelte";
    import Toolbar from "./Toolbar.svelte";
    import { applyStyleToSelection, createElement, getCurrentSelection, removeLineTag, selectLine, setCursorTo, setList, setSelectionAfterThisElement, setTitle, traverseNodeTree } from "./editor";
    let sourceData
    let area;
    let observer;
    let editable=true
    let currentEditStatus='paragraph'

    onMount(() => {
        observer = new MutationObserver((records, ob) => {
            console.log(records);
            console.log(area.childNodes.length)
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

<div class="container mx-auto p-4 w-full h-full">
    <Toolbar
        on:swith_in={(e) => {
            console.log(e.detail)
            const sel=getCurrentSelection(area)
            if(["h1","h2","h3","h4","h5","h6"].includes(e.detail.type)){
                const line=selectLine()
                setTitle(line,e.detail.type)
                area.focus()
                setSelectionAfterThisElement(area)
            }else if(["ol","ul"].includes(e.detail.type)){
                const line=selectLine()
                setList(line,e.detail.type)
            }
            else{
                applyStyleToSelection(sel,e.detail.type)
            }

        }}
        on:switch_out={(e)=>{
            console.log(e.detail)
            if(e.detail.to==="paragraph"){
                let line=selectLine()
                removeLineTag(line)
                // setSelectionAfterThisElement(area)
                area.focus()
            }
        }}
    />
    <div class="relative min-h-fit overflow-hidden">
        
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div on:input={()=>sourceData=area.innerHTML}
            on:keyup={() => {}}
            on:keydown={(e) => {
                if (e.key === "Enter") {
                    // e.preventDefault()
                    
                }
            }}
            contenteditable={editable}
            class="border p-4 inline-block w-full rounded-md h-[200px] mt-4 overflow-auto"
            spellcheck="false"
            bind:this={area}
            id="area"
        >
       <!-- abc<strong>11<i>2345</i>66</strong>def<u>789</u>ghi -->
    </div>
    <div class="border border-dashed mt-4 p-2 rounded-md h-[200px] w-full">{sourceData}</div>
        
    </div>
   
</div>

<style>
    h1{
        font-size: 36px;
    }
</style>
