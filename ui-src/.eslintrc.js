module.exports = {
    root: true,
    env: {
      browser: true,
      es6: true
    },
    extends: [
      "plugin:vue/recommended",
      "eslint:recommended",
      "prettier/vue",
      "plugin:prettier/recommended"
    ],
    rules: {
      "vue/component-name-in-template-casing": ["error", "PacalCase"],
      "no-console": process.env.NODE_ENV === "production" ? "error" : "off",
      "no-debugger": process.env.NODE_ENV === "production" ? "error" : "off"
    },
    parserOptions: {
      parser: "babel-eslint"
    }
  };
  