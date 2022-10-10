module.exports = {
  darkMode: "class",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@brainandbones/skeleton/**/*.{html,js,svelte,ts}",
  ],

  theme: {
    extend: {
      fontFamily: {
        plex: ["IBM Plex Sans", "font-serif"],
        "plex-mono": ["IBM Plex Mono", "font-mono"],
      },
    },
  },

  plugins: [require("@brainandbones/skeleton/tailwind/theme.cjs")],
};
