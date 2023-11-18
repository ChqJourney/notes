import {jwtDecode} from "jwt-decode";

export const decode=(token)=>{
    try{

        const info=jwtDecode(token);
        return info;
    }catch(ex){
        return "";
    }
}

export const isExpired=(info,liveTime)=>{
    return (Date.now()/1000-info.exp)-liveTime>0;
}