import { writable } from "svelte/store";

export const userStore=writable({isAuthenticated:false,userInfo:undefined})

export const getUser=userStore.update(val=>{
    const str=localStorage.getItem("user");
    if(!str){
        val.isAuthenticated=false;
        val.userInfo=undefined;
    }else{
        const user=JSON.parse(str);
        val.isAuthenticated=true;
        val.userInfo={...user}
    }
    return val;
})