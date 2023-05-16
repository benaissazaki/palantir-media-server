module.exports = {
  root: true,
  env: {
    node: true,
    browser: true
  },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: 2021,
    sourceType: "module",
    ecmaFeatures: {
      jsx: true
    },
    project: "./tsconfig.json",
    tsconfigRootDir: __dirname,
  },
  extends: [
    "airbnb-typescript",
    "plugin:react/recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:import/recommended"
  ],
  ignorePatterns: [".eslintrc.cjs", "vite.config.ts"],
  rules: {
    "react/react-in-jsx-scope": "off"
  }
}
