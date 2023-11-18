<script>
// @ts-nocheck

    import {Route} from "svelte-router-spa";
    export let currentRoute;
    const params = {};
</script>

<Route {currentRoute} {params} />
