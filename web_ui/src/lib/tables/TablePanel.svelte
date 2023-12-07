<script>
    import { createEventDispatcher } from "svelte";
    import Pagination from "./Pagination.svelte";
    const dispatch = createEventDispatcher();

    export let rows = [];
    export let columnsConfig = [];
    let currentPage = 1;
    let pageSize = 5;
    let currentSorter;
    $: paginatedData = rows.slice(
        (currentPage-1) * pageSize,
        (currentPage ) * pageSize,
    );

    console.log(rows);
    let selectedIdx = 0;

    function toggleColumnVisibility(columnName) {
        const column = columnsConfig.find((c) => c.name === columnName);
        if (column) {
            column.visible = !column.visible;
        }
    }
    let start, width;

    function initColumnSize(event) {
        event.preventDefault();
        start = event.clientX;
        width = event.target.parentElement.offsetWidth;
        document.addEventListener("mousemove", resizeColumn);
        document.addEventListener("mouseup", stopResizeColumn);
    }

    function resizeColumn(event) {
        let currentWidth = width + event.clientX - start;
        // Update column width
        event.target.parentElement.style.width = `${currentWidth}px`;
    }

    function stopResizeColumn(event) {
        document.removeEventListener("mousemove", resizeColumn);
        document.removeEventListener("mouseup", stopResizeColumn);
    }
    function autoFitColumn(event) {
        const th = event.target.parentElement;
        let maxWidth = 0;
        const rows = th.closest("table").rows;
        console.log(rows);

        for (let row of rows) {
            const cell = row.cells[th.cellIndex];
            const contentWidth = measureContentWidth(cell);
            maxWidth = Math.max(maxWidth, contentWidth);
        }

        th.style.width = `${maxWidth}px`;
    }
    function measureContentWidth(cell) {
        const tempDiv = document.createElement("div");
        tempDiv.style.position = "absolute";
        tempDiv.style.whiteSpace = "nowrap";
        tempDiv.style.visibility = "hidden";
        tempDiv.innerText = cell.innerText;

        document.body.appendChild(tempDiv);
        const width = tempDiv.offsetWidth + 50;
        document.body.removeChild(tempDiv);

        return width;
    }
</script>

<!-- <FTableHeader on:reFetch /> -->
<table
    id="table"
    class="w-full overflow-y-auto h-[85%] table-fixed text-sm text-left text-gray-500 dark:text-gray-400"
>
    <thead class="text-xs text-gray-700 uppercase bg-gray-300">
        <tr class="">
            {#each columnsConfig as config}
                <th scope="col" class={`px-6 justify-around py-4`}>
                    {config.header}
                    <span class="icon-arrow text-lg">
                        <button
                            class={`${currentSorter===config.values[0]?"text-gray-600":"text-gray-400"}`}
                            on:click={() => {
                                config.sorted = !config.sorted;
                                currentSorter=config.values[0]
                                if (config.sorted) {
                                    dispatch("sorting", {
                                        field: config.values[0],
                                        direction: "asc",
                                    });
                                } else {
                                    dispatch("sorting", {
                                        field: config.values[0],
                                        direction: "desc",
                                    });
                                }
                            }}
                        >
                            {`${config.sorted ? "↑" : "↓"}`}
                        </button>
                    </span>
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <span
                        on:mousedown={initColumnSize}
                        on:dblclick={autoFitColumn}
                        class="draggable"
                    ></span>
                </th>
            {/each}
        </tr>
    </thead>
    <tbody class="">
        {#each paginatedData as row, idx}
            <tr
                tabindex={idx}
                class={` border-b ${
                    selectedIdx === idx ? "bg-blue-300" : "bg-white"
                } text-gray-800 cursor-pointer font-sans hover:bg-gray-200`}
                on:click={() => {
                    selectedIdx = idx;
                }}
                on:dblclick={() => dispatch("item_db_click", { row: row })}
            >
                {#each columnsConfig as config}
                    <td class="px-6 py-4 overflow-hidden">
                        {#if config.valueType === "number"}
                            {#if row[config.values[0]] === 0 && row[config.values[1]]}
                                <div>
                                    {row[config.values[1]].toLocaleString(
                                        "zh-CN",
                                        { style: "currency", currency: "CNY" },
                                    )}
                                </div>
                            {:else}
                                <div>
                                    {row[config.values[0]].toLocaleString(
                                        "zh-CN",
                                        { style: "currency", currency: "CNY" },
                                    )}
                                </div>
                            {/if}
                        {:else}
                            <div class=" truncate whitespace-nowrap">
                                {row[config.values[0]]}
                            </div>
                        {/if}
                    </td>
                {/each}
            </tr>
        {/each}
    </tbody>
</table>
<Pagination
    goto={(idx) => (currentPage = idx)}
    previous={() => (currentPage > 1 ? currentPage-- : {})}
    next={() =>
        currentPage < Math.ceil(rows.length / pageSize) ? currentPage++ : {}}
    totalCount={rows.length}
    totalPage={Math.ceil(rows.length / pageSize)}
    {currentPage}
    pageSize={pageSize}
>
    <slot name="summary" />
</Pagination>

<style>
    .table_container::-webkit-scrollbar {
        width: 0.5rem;
        height: 0.5rem;
    }
    .table_container::-webkit-scrollbar-thumb {
        border-radius: 0.5rem;
        background-color: #0004;
        visibility: hidden;
    }

    .table_container:hover::-webkit-scrollbar-thumb {
        visibility: visible;
    }
    tr {
        height: auto;
        overflow: hidden;
    }
    th,
    td {
        /* border: 1px solid black; */
        position: relative;
    }
    .draggable {
        cursor: col-resize;
        padding: 5px;
        color: blueviolet;
        width: 4px;
        position: absolute;
        right: 0;
        top: 0;
        bottom: 0;
    }
</style>
