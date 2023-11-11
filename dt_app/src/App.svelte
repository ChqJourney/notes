<script>
    import { invoke } from "@tauri-apps/api/tauri";

  const open=async()=>{
    await invoke("greet",{name:"ddd"})
  }
</script>
<div class="flex h-24 justify-center items-center">
  app
  <button on:click={open}>open</button>
</div>
