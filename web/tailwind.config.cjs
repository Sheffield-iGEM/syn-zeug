module.exports = {
  darkMode: "class",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}",
  ],

  theme: {
    extend: {
      fontFamily: {
        plex: ["IBM Plex Sans", "font-serif"],
        "plex-mono": ["IBM Plex Mono", "font-mono"],
      },
    },
  },

  plugins: [require("@skeletonlabs/skeleton/tailwind/theme.cjs")],
};
