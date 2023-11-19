<script>
    // @ts-nocheck

    import { Route } from "svelte-router-spa";
    import logo from "../../assets/svelte.svg";
    import { userStore } from "../../stores/userStore";
    import Auth from "../../lib/nav/auth.svelte";
    export let currentRoute;

    const params = {};
</script>

<div class="flex justify-between">
    <img width="40" src={logo} alt="logo" />
    <Auth />
</div>
<hr />
<main>
    <Route {currentRoute} {params} />
</main>
