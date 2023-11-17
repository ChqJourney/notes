<script>
    import Editor from "./lib/editor/Editor.svelte";
  import logo from './assets/svelte.svg'
    import Auth from "./lib/nav/auth.svelte";

</script>

<div class="flex justify-between">
  <img width="40" src={logo} alt="logo"/>
  <Auth/>
</div>
<hr>
<main>
  
  <Editor/>
</main>

<style>
  hr {
      margin: 1rem;
      border: none;
      border-top: 0.5px solid #ccc;
    }
</style>
