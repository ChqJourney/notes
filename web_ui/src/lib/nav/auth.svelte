<script>

    let isAuthenticated=false;
    let userInfo=undefined;
</script>

<div>
    {#if isAuthenticated}
        <a href="#">{userInfo?.email??"error"}</a>
    {:else}
    <div class="flex gap-2">
        <a href="#" class="hover:bg-slate-400 hover:text-lime-300 hover:scale-105 px-2 py-1 rounded-md">Register</a>
        <a href="#" class="hover:bg-slate-400 hover:text-lime-300 hover:scale-105 px-2 py-1 rounded-md">Login</a>
    </div>
    {/if}
</div>