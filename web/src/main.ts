import "@skeletonlabs/skeleton/themes/theme-vintage.css";
import "@skeletonlabs/skeleton/styles/all.css";
import "./app.postcss";
import App from "./App.svelte";
import init from "biobox";

const app = (async () => {
  await init();
  new App({
    target: document.body,
  });
})();

export default app;
