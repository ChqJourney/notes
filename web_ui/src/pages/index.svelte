
<script>
    import { decode, isValid } from "../funcs/authenticate";
    import Editor from "../lib/editor/Editor.svelte";

    let token=""
    let info
    let time
    const send=async ()=>{
        await fetch("https://www.photonee.com/api/identity/register",{
                    method: 'GET',
                    headers: {
                        'X-RapidAPI-Host': 'random-words5.p.rapidapi.com',
                    }
                    // body:JSON.stringify({"dafa":"daf"})
                })
    }
</script>

<div class="p-4">
    
    <Editor/>
</div>