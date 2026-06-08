export default defineNuxtPlugin({
    name: "auth-hooks",
    enforce: "pre",
    parallel: true,
    dependsOn: ["auth-api"],
    setup(nuxtApp) {
        nuxtApp.hooks.addHooks({
            "auth:login": async () => {
                clearNuxtState(["auth_current"]);
                return await navigateTo("/auth/select");
            },
            "auth:select": async () => {
                clearNuxtState(["auth_organizations"]);
                return await navigateTo("/");
            },
            "auth:logout": async () => {
                clearNuxtState(["auth_organizations", "auth_current"]);
                return await navigateTo("/auth/login");
            },
        });
    },
});
