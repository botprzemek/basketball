import { defineNuxtRouteMiddleware, navigateTo, useAuth } from "#imports";

export default defineNuxtRouteMiddleware((_to, _from) => {
    const auth = useAuth();

    if (!auth.isAuthenticated.value) {
        return navigateTo("/");
    }
});
