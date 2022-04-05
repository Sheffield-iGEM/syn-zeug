/** @type {import("snowpack").SnowpackUserConfig } */
export default {
  mount: {
    public: { url: '/', static: true },
    src: { url: '/dist' },
  },
  plugins: [
    '@snowpack/plugin-typescript',
    [
      '@snowpack/plugin-svelte',
      {
        hmrOptions: {
          preserveLocalState: true
        }
      }
    ],
    [
      'snowpack-plugin-wasm-pack',
      { projectPath: '../biobox' }
    ]
  ],
  routes: [
    /* Enable an SPA Fallback in development: */
    // {"match": "routes", "src": ".*", "dest": "/index.html"},
  ],
  optimize: {
    bundle: true,
    minify: true,
    treeshake: true
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    baseUrl: '/syn-zeug'
  },
};
