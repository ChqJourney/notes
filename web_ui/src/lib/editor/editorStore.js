import { readable, writable } from "svelte/store";
export const editorTickStore = writable(0)
export const reset=()=>editorTickStore.set(0)
export const tick=(saveAtFunc)=>editorTickStore.update(val=>{
    if(val===5){
        if(saveAtFunc){
            saveAtFunc()
        }
        return 0
    }else{

        return val+1
    }
})