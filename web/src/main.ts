import "./theme.css";
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
