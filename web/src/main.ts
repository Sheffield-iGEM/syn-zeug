import "./app.postcss";
import App from "./App.svelte";
import init from "biobox";
import '@brainandbones/skeleton/styles/themes/theme-skeleton.css';


const app = (async () => {
  await init();
  new App({
    target: document.body,
  });
})();

export default app;
