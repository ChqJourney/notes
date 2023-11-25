<script>
	export let left = 100;
	export let top = 100;
	
	let moving = false;
	
	function onMouseDown() {
		moving = true;
	}
	
	function onMouseMove(e) {
		if (moving) {
			left += e.movementX;
			top += e.movementY;
		}
	}
	
	function onMouseUp() {
		moving = false;
	}
	
// 	$: console.log(moving);
</script>

<style>
	.draggable {
		user-select: none;
		/* cursor: move; */
		/* border: solid 1px gray; */
		position: absolute;
	}
</style>

<section on:mousedown={onMouseDown} on:dblclick={()=>console.log("db click")} style="left: {left}px; top: {top}px;" class="draggable">
	<slot></slot>
</section>

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} />


<script>
    import Note from './Note.svelte';
  
    let scale = 1;
    let boardRect = { x: 0, y: 0 };
    let translateX = 0, translateY = 0;
  
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
  
    // Other functions for panning (startPan, pan, endPan) go here...
  </script>
  
  <div
    class="board"
    on:wheel={handleWheel}
    bind:this={boardElement}
    style="transform: scale({scale}) translate({translateX}px, {translateY}px);">
    <Note />
    <Note />
    <!-- ... -->
  </div>
  
  <style>
    .board {
      width: 10000px;
      height: 10000px;
      overflow: hidden;
      cursor: grab;
      transform-origin: top left;
    }
  </style>