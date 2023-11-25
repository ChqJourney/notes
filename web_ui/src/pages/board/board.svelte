<script>
    import Note from "../../lib/notes/note.svelte";
    import { notes } from "../../lib/notes/noteStore";
  
  
    let scale = 1;
    let panning=false;
    let boardRect = { x: 0, y: 0 };
    let startX,startY;
    let translateX=0,translateY=0;
  
    function handleWheel(event) {
        const rect = event.currentTarget.getBoundingClientRect();
      const mouseX = event.clientX - rect.left; // x position within the element
      const mouseY = event.clientY - rect.top;  // y position within the element
  
      const oldScale = scale;
      if (event.deltaY < 0) {
        scale *= 1.1;
      } else {
        scale /= 1.1;
      }
  
      // Adjust the translation based on the new scale
      translateX -= (mouseX - boardRect.x) * (scale - oldScale);
      translateY -= (mouseY - boardRect.y) * (scale - oldScale);
    }
    function startPan(event) {
      panning = true;
      startX = event.clientX - translateX;
      startY = event.clientY - translateY;
    }
  
    function pan(event) {
      if (panning) {
        translateX = event.clientX - startX;
        translateY = event.clientY - startY;
      }
    }
  
    function endPan() {
      panning = false;
    }
  </script>
  
  <div
    class="board"
    on:wheel={handleWheel}
    on:mousedown={startPan}
    on:mousemove={pan}
    on:mouseup={endPan}
    on:mouseleave={endPan}
    style="transform: scale({scale}) translate({translateX}px, {translateY}px);">
    {#each $notes as note}
        <Note {note}/>
    {/each}
    <!-- ... -->
  </div>
  
  <style>
    .board {
      width: 10000px;
      height: 10000px;
      overflow: auto;
      cursor: grab;
      transform-origin: top left;
    }
  </style>