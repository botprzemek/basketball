import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  devServer: {
    host: "0.0.0.0",
    port: 3002,
  },

  modules: ["@nuxt/fonts"],

  nitro: {
    runtimeConfig: {
      envPrefix: "BASKETBALL_",
    },
  },

  app: {
    baseURL: "/app/",
  },

  runtimeConfig: {
    apiHost: "http://localhost:3000/api/v1",
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
