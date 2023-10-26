<script>
    import Editor from './lib/editor/Editor.svelte';
import NodeTree from './lib/editor/NodeTree.svelte';
    import PEditor from './lib/editor/PEditor.svelte';
   
let pic;
  const getPic=async()=>{
    pic="http://192.168.0.118:5018/uploads/glass.jpeg"
  }
</script>

<main>
 <PEditor/>
 <div>

  <a href="#" on:click={getPic} >picture</a>
  <img src={pic} alt="dfd"/>

 </div>
</main>
