export default defineNuxtPlugin({
    name: "auth-api",
    enforce: "pre",
    parallel: true,
    dependsOn: [],
    setup: (_nuxtApp) => {
        const headers = useRequestHeaders(["cookie"]);

        const api = $fetch.create({
            baseURL: "http://localhost:3000/api/v1",
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
