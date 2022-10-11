## A Bleeding Edge Web Stack

The core of this web app is [TypeScript](https://www.typescriptlang.org/) enhanced [Svelte](https://svelte.dev/) using the [Skeleton](https://skeleton.brainandbonesllc.com/) UI toolkit, with the computational heavy-lifting done by [Rust](https://www.rust-lang.org/) code compiled to [WebAssembly](https://webassembly.org/) via [`wasm-pack`](https://rustwasm.github.io/wasm-pack/). Gluing all of this together is the [Vite](https://vitejs.dev/) module bundler and the [`pnpm`](https://pnpm.io/) package manager.

## Developing

### Prerequisites

Make sure you have [`rustup`](https://rustup.rs/) installed and have a recent version of [`node`](https://nodejs.org/).

### First Time Setup

```sh
# Install wasm-pack
cargo install wasm-pack
# Enable pnpm
corepack enable
# Install dependencies
pnpm install
# Build the WASM code
pnpm wasm
# Start the development server
pnpm dev
```

### Hot Reloading

While `pnpm dev` will automatically update with any changes you make to your web code, the Rust side of things will need to be manually rebuilt with `pnpm wasm`.
