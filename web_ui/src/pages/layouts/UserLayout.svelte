<script>
    // @ts-nocheck

    import { Route } from "svelte-router-spa";
    import logo from "../../assets/svelte.svg";
    import { userStore } from "../../stores/userStore";
    import Auth from "../../lib/nav/auth.svelte";
    import { editorTickStore } from "../../lib/editor/editorStore";
    export let currentRoute;

    const params = {};

</script>

<div class="flex justify-between px-4 pt-2">
    <img width="40" src={logo} alt="logo" />
    <div>{$editorTickStore}</div>
    <Auth />
</div>
<!-- <hr /> -->
<main class="">
    <Route {currentRoute} {params} />
</main>
