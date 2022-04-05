import App from './App.svelte';
// TODO: I sorta hate the way this works? How can I import individual items
// from this? Rollup is making life a bit difficult?
import wasm from '../../biobox/Cargo.toml';

const app = (async () => {
  const exports = await wasm();
  new App({
    target: document.body,
    props: {
      wasm: exports
    }
  })
})();

export default app;
