/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "**/*.html", "./styles/**/*.s?css"],
  darkMode: "media",
  theme: {
    extend: {
      colors: {
        primary: "#FF5C00",
        secondary: "#FFF5EE",
        outline: "#EDEDED",
        background: "#F8F9FD",
      },
    },
  },
  plugins: [],
};
