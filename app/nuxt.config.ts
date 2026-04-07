import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  devServer: {
    host: "0.0.0.0",
    port: 3002,
  },

  app: {
    baseURL: "/app/",
  },

  nitro: {
    runtimeConfig: {
      envPrefix: "BASKETBALL_",
    },
  },

  modules: ["@nuxt/fonts", "@nuxt/hints", "@nuxt/icon", "@nuxt/image", "@nuxt/test-utils"],

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
