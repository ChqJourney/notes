import "./app.postcss";
import App from "./App.svelte";
import { isExpired } from "./funcs/authenticate";

const { fetch: originalFetch } = window;
window.fetch = async (...args) => {
  let [resource, config] = args;
  // request interceptor
  
  const token_str=localStorage.getItem("user");
  console.log(token_str)
  if(token_str){
    const user=JSON.parse(token_str);
    const {type,access_token,refresh_token,expires_in}=user;
    const is_at_expired=isExpired(user,3600);
    const is_rt_expired=isExpired(user,3600*12);
    if(!is_at_expired){

      config.headers={...config.headers,"Authorization":`${type} ${access_token}`}
    }else if(!is_rt_expired){
      let response=await originalFetch("https://www.photonee.com/api/identity/refresh",{
        method:"POST",
        headers:{"content-type":"application/json"},
        body:JSON.stringify({refresh_token:refresh_token})
      })
      if(response.ok){
        let new_token_info=await response.json()
        localStorage.setItem("user",JSON.stringify(new_token_info))
        const {type,access_token,refresh_token,expires_in}=new_token_info;
        config.headers={...config.headers,"Authorization":`${type} ${access_token}`}
      }
    }
  }
  console.log(config)
  let response
  response = await originalFetch(resource, config);
  // response interceptor
  if(!response.ok&&response.status===401){
   // 401 error handling
      
    return Promise.reject(response);
  }
  if (!response.ok && response.status === 404) {
    // 404 error handling

    return Promise.reject(response);
  }
  return response;
};

const app = new App({
  target: document.getElementById("app"),
});

export default app;
