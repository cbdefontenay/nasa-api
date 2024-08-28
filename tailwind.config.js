/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: 'selector',
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
    fontFamily: {
      "strait": ["Strait-Regular", "strait"],
    }
  },
  plugins: [],
};
