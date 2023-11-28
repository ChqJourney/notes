<script>
    import { notes, selectNote } from "../../lib/notes/noteStore";
    export let note;


    let startX, startY, initialX, initialY;

    function handleMouseDown(event) {
        event.stopPropagation()
        selectNote(note.id)
        startX = event.clientX;
        startY = event.clientY;
        initialX = note.x;
        initialY = note.y;
        
        window.addEventListener('mousemove', handleMouseMove);
        window.addEventListener('mouseup', handleMouseUp);
    }

    function handleMouseMove(event) {
        event.stopPropagation()
        const dx = event.clientX - startX;
        const dy = event.clientY - startY;
        
        note.x = initialX + dx;
        note.y = initialY + dy;
    }

    function handleMouseUp(event) {
        event.stopPropagation()
        window.removeEventListener('mousemove', handleMouseMove);
        window.removeEventListener('mouseup', handleMouseUp);
        dispatch('move', { x: note.x, y: note.y });
        console.log($notes)
    }

    import { createEventDispatcher } from 'svelte';
    import Editor from "../editor/Editor.svelte";
    const dispatch = createEventDispatcher();
</script>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class={`note ${note.zIndex>0?"z-50":"z-0"} text-red-900`} style="left: {note.x}px; top: {note.y}px;" on:mousedown={handleMouseDown}>
    {#if note.status==="card"}
    <div class="">
        <div class={`${note.isSelected?"":"invisible"} flex gap-2 justify-center p-1`}>
            <button class="hover:border hover:border-cyan-700 flex justify-center items-center rounded-md w-8 h-8">
                <svg class="h-6 w-6" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200"><path d="M736 410.272h16a16 16 0 0 1 16 16v413.92a16 16 0 0 1-16 16H208a16 16 0 0 1-16-16v-624a16 16 0 0 1 16-16h394.592a16 16 0 0 1 16 16v16a16 16 0 0 1-16 16H240v560h480v-381.92a16 16 0 0 1 16-16z m78.16-229.6l11.312 11.328a16 16 0 0 1 0 22.624l-316.8 316.8a16 16 0 0 1-22.608 0l-11.312-11.328a16 16 0 0 1 0-22.624l316.784-316.8a16 16 0 0 1 22.624 0zM328 672.208h304a16 16 0 0 1 16 16v16a16 16 0 0 1-16 16h-304a16 16 0 0 1-16-16v-16a16 16 0 0 1 16-16z m8-104h64a16 16 0 0 1 16 16v16a16 16 0 0 1-16 16h-64a16 16 0 0 1-16-16v-16a16 16 0 0 1 16-16z" fill="#1296db"></path></svg>
            </button>
            <button class="hover:border hover:border-cyan-700 flex justify-center items-center rounded-md w-8 h-8">
                <svg class="h-5 w-5" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200"><path d="M924.43026 238.663658 770.369466 238.663658l0-104.254435c0-38.566364-31.397081-69.990051-69.963445-69.990051L323.606259 64.419172c-38.566364 0-69.949118 31.423687-69.949118 69.990051l0 104.3363-154.334018 0.054235c-9.286504 0-18.013259 3.619434-24.595164 10.228969-6.568602 6.5553-10.188037 15.308661-10.160407 24.581861 0 19.173688 15.59621 34.81083 34.783201 34.81083l78.594009-0.013303L177.944761 889.430118c0 38.566364 31.382754 69.990051 69.963445 69.990051l528.225543 0c38.566364 0 69.963445-31.423687 69.963445-69.990051L846.097194 330.860477l-0.163729 0-0.013303-22.560832 78.539774-0.013303c19.188015 0 34.783201-15.637142 34.783201-34.851763C959.213461 254.259868 943.603949 238.663658 924.43026 238.663658zM412.347372 822.007543c-19.188015 0-34.783201-15.637142-34.783201-34.81083L377.399419 414.779771c0-19.173688 15.59621-34.824133 34.797527-34.824133 19.188015 0 34.783201 15.650445 34.783201 34.824133l0.163729 372.361683C447.143876 806.316166 431.521061 821.966611 412.347372 822.007543zM611.842962 822.007543c-19.201318 0-34.81083-15.637142-34.81083-34.81083L576.868403 414.779771c0-19.173688 15.59621-34.824133 34.783201-34.824133 19.201318 0 34.797527 15.650445 34.797527 34.824133l0.163729 372.361683C646.627187 806.316166 631.030977 821.966611 611.842962 822.007543zM323.401598 177.427992c0-23.844058 19.405979-43.210128 43.223431-43.210128l290.763247 0c23.844058 0 43.25106 19.365046 43.25106 43.210128l0 61.277622-377.237737 0.040932L323.401598 177.427992z" fill="#1296db"></path></svg>
            </button>
            <button class="hover:border hover:border-cyan-700 flex justify-center items-center rounded-md w-8 h-8">
                <svg class="h-4 w-4" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200"><path d="M896 1024H544a32 32 0 0 1 0-64h352a64 64 0 0 0 64-64V128a64 64 0 0 0-64-64H128a64 64 0 0 0-64 64v352a32 32 0 0 1-64 0V128a128 128 0 0 1 128-128h768a128 128 0 0 1 128 128v768a128 128 0 0 1-128 128zM128 576h192a128 128 0 0 1 128 128v192a128 128 0 0 1-128 128H128a128 128 0 0 1-128-128v-192a128 128 0 0 1 128-128zM64 896a64 64 0 0 0 64 64h192a64 64 0 0 0 64-64v-192a64 64 0 0 0-64-64H128a64 64 0 0 0-64 64v192zM754.752 224H544a32 32 0 0 1 0-64h288a32 32 0 0 1 32 32v288a32 32 0 0 1-64 0V269.248l-265.376 265.376a32 32 0 0 1-45.248-45.248z" fill="#1296db"></path></svg>
            </button>
        </div>
        <div class={`w-48 h-48 mt-2 bg-slate-400 dark:text-white border ${note.isSelected?"border-blue-800":"border-red-800"}`} on:dblclick={()=>note.status="editor"}>{note.textContent}</div>
    </div>
    {:else}
    <div class="block">
        <div class={`${note.isSelected?"":"invisible"} flex gap-2 justify-end p-1`}>
            <button class="hover:border hover:border-cyan-700 flex justify-center items-center rounded-md w-8 h-8">
                <svg class="h-5 w-5" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200"><path d="M437.333333 224C437.333333 183.466667 471.466667 149.333333 512 149.333333s74.666667 34.133333 74.666667 74.666667S552.533333 298.666667 512 298.666667s-74.666667-34.133333-74.666667-74.666667zM439.466667 512c0-40.533333 34.133333-74.666667 74.666666-74.666667s74.666667 34.133333 74.666667 74.666667-34.133333 74.666667-74.666667 74.666667-74.666667-34.133333-74.666666-74.666667zM437.333333 800c0-40.533333 34.133333-74.666667 74.666667-74.666667s74.666667 34.133333 74.666667 74.666667S552.533333 874.666667 512 874.666667s-74.666667-34.133333-74.666667-74.666667z" fill="#1296db"></path></svg>
            </button>
            <button class="hover:border hover:border-cyan-700 flex justify-center items-center rounded-md w-8 h-8">
                <svg class="h-4 w-4" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200"><path d="M896 1024H544a32 32 0 0 1 0-64h352a64 64 0 0 0 64-64V128a64 64 0 0 0-64-64H128a64 64 0 0 0-64 64v352a32 32 0 0 1-64 0V128a128 128 0 0 1 128-128h768a128 128 0 0 1 128 128v768a128 128 0 0 1-128 128zM128 576h192a128 128 0 0 1 128 128v192a128 128 0 0 1-128 128H128a128 128 0 0 1-128-128v-192a128 128 0 0 1 128-128zM64 896a64 64 0 0 0 64 64h192a64 64 0 0 0 64-64v-192a64 64 0 0 0-64-64H128a64 64 0 0 0-64 64v192zM754.752 224H544a32 32 0 0 1 0-64h288a32 32 0 0 1 32 32v288a32 32 0 0 1-64 0V269.248l-265.376 265.376a32 32 0 0 1-45.248-45.248z" fill="#1296db"></path></svg>
            </button>
            <button class="hover:border hover:border-cyan-700 flex justify-center items-center rounded-md w-8 h-8">
                <svg class="h-6 w-6" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" width="200" height="200"><path d="M557.312 513.248l265.28-263.904c12.544-12.48 12.608-32.704 0.128-45.248-12.512-12.576-32.704-12.608-45.248-0.128L512.128 467.904l-263.04-263.84c-12.448-12.48-32.704-12.544-45.248-0.064-12.512 12.48-12.544 32.736-0.064 45.28l262.976 263.776L201.6 776.8c-12.544 12.48-12.608 32.704-0.128 45.248a31.937 31.937 0 0 0 22.688 9.44c8.16 0 16.32-3.104 22.56-9.312l265.216-263.808 265.44 266.24c6.24 6.272 14.432 9.408 22.656 9.408a31.94 31.94 0 0 0 22.592-9.344c12.512-12.48 12.544-32.704 0.064-45.248L557.312 513.248z" fill="#1296db"></path></svg>
            </button>
        </div>
        <div class={`w-[600px] h-96 border rounded-md ${note.isSelected?"border-blue-800":"border-red-800"} overflow-hidden`}>
            <Editor htmlConent={note.htmlContent}/>
        </div>
    </div> 
    {/if}
</div>

<style>
    .note {
        position: absolute;
        cursor: grab;
        border-radius: 0.5rem;
        padding: 1px;
        /* Add more styles as needed */
    }
</style>