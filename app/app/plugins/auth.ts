export default defineNuxtPlugin({
    name: "auth-hooks",
    enforce: "pre",
    parallel: true,
    dependsOn: ["auth-api"],
    setup(nuxtApp) {
        nuxtApp.hooks.addHooks({
            "auth:login": async () => {
                await navigateTo("/auth/select");
                clearNuxtState(["auth_current"]);
            },
            "auth:select": async () => {
                await navigateTo("/");
                clearNuxtState(["auth_organizations"]);
            },
            "auth:logout": async () => {
                await navigateTo("/auth/login");
                clearNuxtState(["auth_organizations", "auth_current"]);
            },
        });
    },
});
