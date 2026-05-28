import { fileURLToPath, URL } from "node:url";
import {
    defineNuxtModule,
    createResolver,
    addImportsDir,
    addServerHandler,
    addPlugin,
    addRouteMiddleware,
} from "@nuxt/kit";

export interface ModuleHooks {
    "auth:login": (to: string) => void;
    "auth:logout": (to: string) => void;
}

export interface ModuleOptions {}

export default defineNuxtModule<ModuleOptions>({
    meta: {
        name: "basketball-auth",
        configKey: "auth",
    },
    defaults: {},
    moduleDependencies: {},
    setup(_options, _nuxt) {
        const resolver = createResolver(import.meta.url);
        const runtime = fileURLToPath(new URL("./runtime", import.meta.url));

        // nuxt.options.css.push(
        //     resolver.resolve(runtime, "assets/css/main.css"),
        // );

        addServerHandler({
            route: "/api/auth/login",
            handler: resolver.resolve(runtime, "server/api/auth/login.post"),
        });
        addServerHandler({
            route: "/api/auth/me",
            handler: resolver.resolve(runtime, "server/api/auth/me.get"),
        });
        addServerHandler({
            route: "/api/auth/logout",
            handler: resolver.resolve(runtime, "server/api/auth/logout.post"),
        });

        addImportsDir(resolver.resolve(runtime, "composables"));

        addPlugin(resolver.resolve(runtime, "plugins/auth"));

        addRouteMiddleware({
            name: "auth",
            path: resolver.resolve(runtime, "middleware/auth"),
        });
    },
});
