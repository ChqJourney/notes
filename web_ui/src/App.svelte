<script>
// @ts-nocheck

  import { Router } from 'svelte-router-spa'
  import { routes } from './routes'
</script>

<Router {routes} />

<style>
  hr {
      margin: 1rem;
      border: none;
      border-top: 0.5px solid #ccc;
    }
</style>
