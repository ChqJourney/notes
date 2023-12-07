<script>
    export let totalCount,totalPage,currentPage,pageSize;
    export let next,previous,goto;
    console.log(totalPage)
    $:indexArr=function getPaginationRange() {
    let range = [];
    let start = Math.max(1, currentPage - 2);
    let end = Math.min(totalPage, currentPage + 2);
    console.log(start,end)
    
      if (start >= 2) {
        range.push('...');
      }
    

    for (let i = start; i <= end; i++) {
      range.push(i);
    }

    if (end < totalPage) {
      if (end < totalPage - 1) {
        range.push('...');
      }
      range.push(totalPage);
    }
    console.log(range)
    return range;
  }()
    // $:indexArr=Array.from({length:totalPage},(v,k)=>{
    //     if(totalPage>10){
    //         if(k+1<=currentPage+2&&k+1>=currentPage-2){
    //             return k+1;
    //         }else if(k+1<=2||k+1>=totalPage-2){
    //             return k+1;
    //         }else{
    //             return 0;
    //         }
    //     }else{
    //         return k+1;
    //     }
    // });
</script>
<nav class="flex items-center justify-between pt-4 px-2" aria-label="Table navigation">
    <span class="text-sm font-normal dark:text-white text-gray-500">Showing <span class="font-semibold text-gray-900 dark:text-white ">{(currentPage-1)*pageSize+1}-{totalCount<currentPage*pageSize?totalCount:currentPage*pageSize}</span> of <span class="font-semibold text-gray-900 dark:text-white">{totalCount}</span></span>
    <slot></slot>
    <ul class="inline-flex items-center -space-x-px">
        <li>
            <a href="#" on:click={previous} class="block px-3 py-2 ml-0 leading-tight text-gray-500 bg-white border border-gray-300 rounded-l-lg hover:bg-gray-100 hover:text-gray-700">
                <span class="sr-only">Previous</span>
                <svg class="w-5 h-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
            </a>
        </li>
        
        {#each indexArr as index}
        {#if index==="..."}
        <span>...</span>
        {:else}
            <li>
                <a href="#" on:click={goto(index)} aria-current="page" class={`z-10 px-3 py-2 leading-tight ${index===currentPage?"border border-pink-300 text-blue-600 bg-blue-200 hover:bg-blue-100 hover:text-blue-700":"border border-gray-400 text-gray-700 bg-gray-100 hover:bg-gray-200 hover:text-gray-800"}  `}>{index}</a>
            </li>
            
            {/if}
        {/each}
        <li>
            <a href="#" on:click={next} class="block px-3 py-2 leading-tight text-gray-500 bg-white border border-gray-300 rounded-r-lg hover:bg-gray-100 hover:text-gray-700 ">
                <span class="sr-only">Next</span>
                <svg class="w-5 h-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"></path></svg>
            </a>
        </li>
    </ul>
</nav>
<style>
    .add-dot::before{
        content: "...";
        margin-left: 10px;
        margin-right: 10px;
    }
    .follow-dot::after{
        content: "...";
        margin-left: 10px;
        margin-right: 10px;
    }
</style>