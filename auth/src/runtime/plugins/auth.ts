import { defineNuxtPlugin, navigateTo, useAuth } from "#imports";

export default defineNuxtPlugin({
    name: "auth",
    enforce: "pre",
    parallel: true,
    async setup(_nuxt) {
        const { restore } = useAuth();

        await restore();
    },
    hooks: {
        "auth:login": async (to: string) => {
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
