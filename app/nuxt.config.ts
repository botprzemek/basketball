import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
    compatibilityDate: "2025-07-15",
    devtools: {
        enabled: true,
        timeline: {
            enabled: true,
        },
    },

    modules: ["@nuxt/fonts"],

    nitro: {
        runtimeConfig: {
            envPrefix: "AUTH_",
        },
    },

    runtimeConfig: {
        apiHost: "",
        tokens: {
            identity: "identity-token",
            access: "access-token",
            refresh: "refresh-token",
        },
    },

    vite: {
        plugins: [tailwindcss()],
        server: {
            allowedHosts: true,
        },
        optimizeDeps: {
            include: ["@vue/devtools-core", "@vue/devtools-kit"],
        },
    },

    css: ["./app/assets/css/main.css"],
});
