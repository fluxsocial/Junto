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
      "vue/component-name-in-template-casing": ["error", "PascalCase", {
        "registeredComponentsOnly": true,
        "ignores": []
      }],
      "no-console": process.env.NODE_ENV === "production" ? "error" : "off",
      "no-debugger": process.env.NODE_ENV === "production" ? "error" : "off",
      "indent": 1
    },
    parserOptions: {
      parser: "babel-eslint"
    }
  };
  