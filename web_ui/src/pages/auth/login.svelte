<script>
    import { getUser } from "../../stores/userStore";
    import {navigateTo} from "svelte-router-spa"
    let email
    let password
</script>

<!--
  This example requires some changes to your config:
  
  ```
  // tailwind.config.js
  module.exports = {
    // ...
    plugins: [
      // ...
      require('@tailwindcss/forms'),
    ],
  }
  ```
-->
<!--
  This example requires updating your template:

  ```
  <html class="h-full bg-gray-50">
  <body class="h-full">
  ```
-->
<div class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-md">
      <img class="mx-auto h-12 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company">
      <h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900 dark:text-white">Sign in to your account</h2>
      <!-- <p class="mt-2 text-center text-sm text-gray-600">
        Or
        <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">start your 14-day free trial</a>
      </p> -->
    </div>
  
    <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
      <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
        <form class="space-y-6" >
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700">Email address</label>
            <div class="mt-1">
              <input bind:value={email} id="email" name="email" type="email" autocomplete="email" required class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-indigo-500 focus:outline-none text-red-400 focus:ring-indigo-500 sm:text-sm">
            </div>
          </div>
  
          <div>
            <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
            <div class="mt-1">
              <input bind:value={password} id="password" name="password" type="password" autocomplete="current-password" required class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 text-red-400 placeholder-gray-400 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm">
            </div>
          </div>
  
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <input id="remember-me" name="remember-me" type="checkbox" class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500">
              <label for="remember-me" class="ml-2 block text-sm text-gray-900">Remember me</label>
            </div>
  
            <div class="text-sm">
              <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">Forgot your password?</a>
            </div>
          </div>
  
          <div>
            <button type="submit" on:click={async(e)=>{
              e.preventDefault()
              const response=await fetch("https://www.photonee.com/api/identity/login",{
                method: 'POST',
                    headers: {
                        'content-type': 'application/json',
                    },
                    body:JSON.stringify({email:email,password:password})
              });
              let res=await response.json();
              console.log(res);
              localStorage.setItem("user",JSON.stringify(res));
              getUser()
              navigateTo("/");
            }} class="flex w-full justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Sign in</button>
          </div>
        </form>
  
        <div class="mt-6">
          
  
          <div class="mt-6 flex justify-center font-medium text-emerald-600 hover:text-emerald-700">
             <a href="#" on:click={()=>navigateTo("/register")} class="hover:scale-105 underline ">register a new account</a>
          </div>
        </div>
      </div>
    </div>
  </div>