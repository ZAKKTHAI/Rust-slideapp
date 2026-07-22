/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{ts,js}"],
  theme: {
    extend: {},
  },
  plugins: [require("@digital-go-jp/tailwind-theme-plugin")],
};
