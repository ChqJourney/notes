<script>
    import FunBtns from "../../lib/nav/funBtns.svelte";
import SNote from "../../lib/notes/sNote.svelte";

const zoome=(e)=>{
    
}
</script>

<div on:wheel={e=>zoome(e)} class=" block w-full h-[500px] relative mx-2 overflow-hidden">
    <div class="absolute top-[50%] left-[5%]">
        <FunBtns/>
    </div>
    <SNote>
        <div class="w-36 h-36 border border-purple-600 rounded-md">notes here</div>
    </SNote>
</div>