import "./app.postcss";
import App from "./App.svelte";

const { fetch: originalFetch } = window;
window.fetch = async (...args) => {
  let [resource, config] = args;
  // request interceptor
  const token="dfadfadfad";
  config.headers={...config.headers,"Authorization":`Bearer ${token}`}
  console.log(config)
  let response = await originalFetch(resource, config);
  // response interceptor

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
