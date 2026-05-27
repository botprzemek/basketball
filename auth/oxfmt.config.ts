import { defineConfig } from "oxfmt";

export default defineConfig({
    printWidth: 80,
    tabWidth: 4,
    useTabs: false,
    semi: true,
    singleQuote: false,
    trailingComma: "all",
    ignorePatterns: [],
    sortImports: {
        newlinesBetween: false,
        partitionByNewline: true,
    },
    sortTailwindcss: {
        preserveDuplicates: false,
    },
    sortPackageJson: {
        sortScripts: false,
    },
});
