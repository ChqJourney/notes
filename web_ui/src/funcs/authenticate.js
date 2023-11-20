import {jwtDecode} from "jwt-decode";

export const decode=(token)=>{
    try{

        const info=jwtDecode(token);
        console.log(info)
        return info;
    }catch(ex){
        return "";
    }
}

export const isValid=(info,liveTime)=>{
    console.log(info.exp)
    console.log(Date.now()/1000)
    let result=liveTime-(Date.now()/1000-info.exp)
    console.log(result)
    return result>0;
}