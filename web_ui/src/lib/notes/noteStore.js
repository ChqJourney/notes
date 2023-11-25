import { writable } from "svelte/store";

export const notes=writable([
    {id:1,x:100,y:100,content:"343",isSelected:true},
    {id:2,x:200,y:100,content:"34uuh3",isSelected:false},
    {id:3,x:200,y:200,content:"34,kmn3",isSelected:false},
]);
export const selectNote=(id)=>notes.update(val=>{
    return val.map(v=>{
        if(v.id===id){
            return {...v,isSelected:true}
        }else{
            return {...v,isSelected:false}
        }
    })
})