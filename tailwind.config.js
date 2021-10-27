const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

module.exports = {
  darkMode: "media", // or 'media' or 'class'
  purge: ["./web/templates/*.html", "./web/*.css", "./web/*.es6"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Fira Sans", ...defaultTheme.fontFamily.sans],
      },
    },
    colors: {
      transparent: "transparent",
      primary: {
        light: colors.cyan["300"], // ?
        DEFAULT: colors.cyan["400"], // #22D3EE
        dark: colors.cyan["500"], // ?
      },
      secondary: {
        light: colors.blueGray["600"], // ?
        DEFAULT: colors.blueGray["700"], // #334155
        dark: colors.blueGray["800"], // ?
      },
      accent: {
        DEFAULT: colors.fuchsia["500"], // #D946EF
      },
      gray: {
        light: colors.gray["200"], // #E4E4E7
        DEFAULT: colors.gray["400"], // #A1A1AA
        dark: colors.gray["600"], // #52525B
      },
      blue: {
        DEFAULT: "#2196f3",
      },
      red: {
        DEFAULT: "#f44336",
      },
      green: {
        DEFAULT: "#4caf50",
      },
      yellow: {
        DEFAULT: "#ffeb3b",
      },
    },
    textColor: {
      primary: colors.gray["600"], // #52525B
      secondary: colors.gray["200"], // #E4E4E7
      danger: colors.red["600"], // ?
      light: colors.gray["200"], // #A1A1AA
      gray: colors.gray["400"], // #A1A1AA
      dark: colors.blueGray["800"], // #A1A1AA
      blue: colors.blue["700"], // ?
      red: colors.red["700"], // ?
      green: colors.green["700"], // ?
      yellow: colors.yellow["700"], // ?
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
