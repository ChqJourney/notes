<script>
    import { onMount } from "svelte";
    let quillEditor;
    

    onMount(async()=>{
        import("quill/dist/quill.bubble.css");
        const quillModule=await import("quill/dist/quill");
        const Quill=quillModule.default;
        quillEditor=new Quill("#editor",{
            modules:{
                toolbar:[
                    [{header:[1,2,false]}],
                    ["bold","italic","underline"],
                    ["image","code-block"]
                ]
            },
            placeholder:"text here...",
            theme:"bubble"
        })
    });

</script>

<div id="editor" class="border placeholder-slate-50 dark:text-white text-gray-800"></div>

<style>
    .quill > .ql-container > .ql-editor.ql-blank::before{
    font-size: 20px;
    color: red;
}
</style>