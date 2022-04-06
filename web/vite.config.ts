import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasmPack from 'vite-plugin-wasm-pack'

// https://vitejs.dev/config/
export default defineConfig({
  base: '/syn-zeug/',
  plugins: [svelte(), wasmPack('./biobox')]
})
