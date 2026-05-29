import { defineNuxtPlugin } from "#imports";

export default defineNuxtPlugin((_nuxtApp) => {
    const api = $fetch.create({
        baseURL: "http://localhost:3000/api/v1",
    });

    return {
        provide: {
            api,
        },
    };
});
