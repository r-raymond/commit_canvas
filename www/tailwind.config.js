/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html"],
  theme: {
    fontFamily: {
      sans: ["Rock Salt"],
    },
    extend: {
      colors: {
        primary: "#FCA5A5",
      },
      lineHeight: {
        0: "0",
      },
    },
  },
  plugins: [],
};
