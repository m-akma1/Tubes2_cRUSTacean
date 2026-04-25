/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        brand: {
          teal: "#067BC2",
          sky: "#84BCDA",
          olive: "#B8C073",
          amber: "#ECC30B",
          coral: "#F37748",
          lobster: "#D56062",
        },
        accent: "#067BC2",
        match: "#B8C073",
        visited: "#ECC30B",
        surface: {
          950: "#fffdf4",
          900: "#fff7df",
          800: "#f8efd2",
          700: "#eadfbf",
          600: "#cbbf9b",
        },
      },
      fontFamily: {
        sans: ["Inter", "ui-sans-serif", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "ui-monospace", "monospace"],
      },
    },
  },
  plugins: [],
};
