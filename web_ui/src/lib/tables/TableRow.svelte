<script>
    import { createEventDispatcher, tick } from "svelte";
    const dispatch = createEventDispatcher();
    export let configs,row,idx;
    export let selectedIdx
</script>

<tr  style="height: 40px;" tabindex={idx} 
                    class={` border-b ${selectedIdx===idx?"bg-blue-300":"bg-white"} text-gray-800 cursor-pointer font-sans hover:bg-gray-200`} 
                    on:click={()=>{
                        dispatch("selected",{index:idx})
                        // row.selected=!row.selected;
                        // row={...row}
                        // if(row.selected){

                        //     dispatch("selected",{id:row.Id})
                        // }
                    }}
                    on:dblclick={() => dispatch("item_db_click", { row: row })}
                >
                   
                    {#each configs as config}
                        <td class="px-6 py-4 overflow-hidden">
                           
                                {#if config.valueType === "number"}
                                    {#if row[config.values[0]]===0&&row[config.values[1]]}
                                     <div>{row[config.values[1]].toLocaleString('zh-CN',{style:'currency',currency:"CNY"})}</div> 
                                    {:else}
                                      <div>{row[config.values[0]].toLocaleString('zh-CN',{style:'currency',currency:"CNY"})}</div>
                                    {/if}
                                {:else}
                                    <div class=" truncate">{row[config.values[0]]}</div>
                                {/if}
                           
                        </td>
                    {/each}
                   
                </tr>