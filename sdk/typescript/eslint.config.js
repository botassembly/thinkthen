import tseslint from "typescript-eslint";

export default tseslint.config(
  tseslint.configs.strictTypeChecked,
  {
    files: ["runtime.js", "bridge.js", "stream.js"],
    languageOptions: {
      parserOptions: { project: "./tsconfig.json", tsconfigRootDir: import.meta.dirname },
    },
  },
);
