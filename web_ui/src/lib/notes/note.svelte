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
    const dispatch = createEventDispatcher();
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class={`note w-48 h-48 dark:text-white border ${note.isSelected?"border-blue-800":"border-red-800"} text-red-900`} style="left: {note.x}px; top: {note.y}px;" on:mousedown={handleMouseDown}>
    {note.content}
</div>

<style>
    .note {
        position: absolute;
        cursor: grab;
        
        /* Add more styles as needed */
    }
</style>