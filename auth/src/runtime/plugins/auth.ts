import { defineNuxtPlugin, navigateTo } from "#imports";

export default defineNuxtPlugin({
    name: "auth",
    enforce: "pre",
    parallel: true,
    hooks: {
        "auth:login": async (to: string) => {
            await navigateTo(to);
        },
        "auth:restore": async (to: string) => {
            await navigateTo(to);
        },
        "auth:logout": async (to: string) => {
            await navigateTo(to);
        },
    },
    env: {
        islands: true,
    },
});
