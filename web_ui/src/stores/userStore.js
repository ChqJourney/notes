import { writable } from "svelte/store";
import { decode, isExpired } from "../funcs/authenticate";

export const userStore = writable({ isAuthenticated: false, userInfo: undefined })

export const getUser = () => userStore.update(val => {
    const str = localStorage.getItem("user");
    if (!str) {
        val.isAuthenticated = false;
        val.userInfo = undefined;
    } else {
        const user = JSON.parse(str);
        val.isAuthenticated = true;
        const info = decode(user.access_token)
        val.userInfo = { ...info }
    }
    return val;
})

export const checkAndRefreshAuthState = () => {
    const str = localStorage.getItem("user");
    console.log(str)
    if (!str) {
        userStore.update(val => { val.isAuthenticated = false; val.userInfo = undefined; return val; })
        return false;
    } else {
        const user = JSON.parse(str);
        const info = decode(user.access_token)
        const is_exp = isExpired(info, 3600 * 1000)
        if (is_exp) {
            userStore.update(val => { val.isAuthenticated = true; val.userInfo = { ...user }; return val; })
        } else {
            userStore.update(val => { val.isAuthenticated = false; val.userInfo = undefined; return val; })
        }
        return is_exp
    }
}