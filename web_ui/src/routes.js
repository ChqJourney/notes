import Login from "./pages/auth/login.svelte"
import Register from "./pages/auth/register.svelte"
import Board from "./pages/board/board.svelte"
import Db from "./pages/db/db.svelte"
import Index from "./pages/index.svelte"
import TopLayout from "./pages/layouts/TopLayout.svelte"
import UserLayout from "./pages/layouts/UserLayout.svelte"
import { getUser, checkAndRefreshAuthState, userStore } from "./stores/userStore"

let is_authed=false;
function userIsAdmin() {
    getUser()
    userStore.subscribe(val=>is_authed=val.isAuthenticated)
  //check if user is admin and returns true or false
}

const routes = [
  {
    name: '/',
    component: Index,
    layout:UserLayout,
    onlyIf: { guard: checkAndRefreshAuthState, redirect: '/login' },
    nestedRoutes:[

    ]
  },
  {
    name:'/board',
    component:Board,
    layout:UserLayout,
    onlyIf: { guard: checkAndRefreshAuthState, redirect: '/login' },
  },
  {
    name:'/db',
    component:Db,
    layout:UserLayout,
    onlyIf: { guard: checkAndRefreshAuthState, redirect: '/login' },
  },
  { name: 'login', component: Login, layout: TopLayout },
  {name:"register",component:Register,layout:TopLayout},
]

export { routes }