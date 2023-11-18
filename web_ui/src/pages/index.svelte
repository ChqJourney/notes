
<script>
    import { decode, isExpired } from "../funcs/authenticate";

    let token=""
    let info
    let time
    const send=async ()=>{
        await fetch("https://www.photonee.com/api/identity/register",{
                    method: 'GET',
                    headers: {
                        'X-RapidAPI-Host': 'random-words5.p.rapidapi.com',
                    },
                    // body:JSON.stringify({"dafa":"daf"})
                })
    }
</script>


<div>index</div>
<button class="bg-slate-300 w-24 h-8 rounded-md" on:click={send}>Send</button>
