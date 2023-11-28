import { writable } from "svelte/store";

export const notes=writable([
    {id:1,x:5100,y:5100,zIndex:50,htmlContent:"343",textContent:"ddd2323fdf",isSelected:true,status:"card"},
    {id:2,x:5200,y:5100,zIndex:0,htmlContent:"343",textContent:"dfd dfdf",isSelected:false,status:"card"},
    {id:3,x:5200,y:5200,zIndex:0,htmlContent:"343",textContent:"dfd nnn mmmf",isSelected:false,status:"card"},
]);
export const selectNote=(id)=>notes.update(val=>{
    return val.map(v=>{
        if(v.id===id){
            return {...v,isSelected:true,zIndex:50}
        }else{
            return {...v,isSelected:false,zIndex:0}
        }
    })
})