export default defineNuxtPlugin({
    name: "auth-api",
    enforce: "pre",
    parallel: true,
    dependsOn: [],
    setup: (_nuxtApp) => {
        const { apiUrl } = useRuntimeConfig();
        const headers = useRequestHeaders(["cookie"]);

        const api = $fetch.create({
            baseURL: apiUrl,
            credentials: "include",
            headers,
        });

        return {
            provide: {
                api,
            },
        };
    },
});
