import Login from "./pages/auth/login.svelte"
import Index from "./pages/index.svelte"
import TopLayout from "./pages/layouts/TopLayout.svelte"


function userIsAdmin() {
    let str=localStorage.getItem("user")
    if(!str){return false}else{
        let user=JSON.parse(str)
    }
  //check if user is admin and returns true or false
}

const routes = [
  {
    name: '/',
    component: Index,
    layout:TopLayout,
    // onlyIf: { guard: userIsAdmin, redirect: '/login' },
    // nestedRoutes:[

    // ]
  },
  { name: 'login', component: Login, layout: TopLayout },
  
]

export { routes }