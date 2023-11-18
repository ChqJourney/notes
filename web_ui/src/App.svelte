<script>
  import { Router } from 'svelte-router-spa'
  import { routes } from './routes'
</script>

<Router {routes} />


<!-- <div class="flex justify-between">
  <img width="40" src={logo} alt="logo"/>
  <Auth/>
</div>
<hr>
<main>
  
  <Editor/>
</main> -->

<style>
  hr {
      margin: 1rem;
      border: none;
      border-top: 0.5px solid #ccc;
    }
</style>
