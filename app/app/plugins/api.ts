export default defineNuxtPlugin({
    name: "auth-api",
    enforce: "pre",
    parallel: true,
    dependsOn: [],
    setup: (_nuxtApp) => {
        const { apiUrl } = useRuntimeConfig().public;
        const headers = useRequestHeaders(["cookie"]);

        const api = $fetch.create({
            baseURL: `${apiUrl}/v1`,
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
