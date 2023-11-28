import { writable } from "svelte/store";

export const notes=writable([
    {id:1,x:5100,y:5100,zIndex:50,htmlContent:"<h2>title</h2><s><strong>bold</strong></s><strong>china di dk kdf kdfk k dk fdfk dfk;adf;kjdfd;</strong>",textContent:"ddd2323fdf\n dfaf adfaf adfa; 9dfd 9 dfdf\n a;dkf98fd df9898 fda",isSelected:true,status:"card"},
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
export const setNoteStatus=(status,id)=>notes.update(val=>{
    return val.map(v=>{
        if(v.id===id){
            return {...v,status:status}
        }else{
            return v
        }
    })
})