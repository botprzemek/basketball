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
    setup(_options, _nuxt) {
        const resolver = createResolver(import.meta.url);

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
