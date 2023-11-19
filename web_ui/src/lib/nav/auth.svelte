<script>
    import {navigateTo} from "svelte-router-spa";
    import {userStore} from "../../stores/userStore"
    console.log($userStore)
</script>

<div>
    {#if $userStore.isAuthenticated}
        <a href="#">{$userStore.userInfo?.email??"error"}</a>
    {:else}
    <div class="flex gap-2">
        <a href="#" on:click={()=>navigateTo("/register")} class="hover:bg-slate-400 hover:text-lime-300 hover:scale-105 px-2 py-1 rounded-md">Register</a>
        <a href="#" on:click={()=>navigateTo("/login")} class="hover:bg-slate-400 hover:text-lime-300 hover:scale-105 px-2 py-1 rounded-md">Login</a>
    </div>
    {/if}
</div>