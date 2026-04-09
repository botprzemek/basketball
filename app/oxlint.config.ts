import { defineConfig } from "oxlint";

export default defineConfig({
    plugins: ["typescript", "vue"],
    categories: {
        correctness: "warn",
    },
    rules: {
        "eslint/no-unused-vars": "error",
    },
});
