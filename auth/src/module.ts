import {
    defineNuxtModule,
    createResolver,
    addImportsDir,
    addServerHandler,
} from "@nuxt/kit";

export interface ModuleOptions {}

export default defineNuxtModule<ModuleOptions>({
    meta: {
        name: "basketball-auth",
        configKey: "auth",
    },

    defaults: {},
    moduleDependencies: {
        "@nuxt/ui": {
            version: ">=4",
        },
    },
    setup(_options, nuxt) {
        const resolver = createResolver(import.meta.url);

        nuxt.options.css.push(
            resolver.resolve("./runtime/app/assets/css/main.css"),
        );

        addImportsDir(resolver.resolve("./runtime/app/composables"));

        addServerHandler({
            route: "/api/auth/login",
            handler: resolver.resolve("./runtime/server/api/auth/login.post"),
        });
        addServerHandler({
            route: "/api/auth/me",
            handler: resolver.resolve("./runtime/server/api/auth/me.get"),
        });
        addServerHandler({
            route: "/api/auth/logout",
            handler: resolver.resolve("./runtime/server/api/auth/logout.post"),
        });
    },
});
