import { defineNuxtRouteMiddleware, navigateTo, useAuth } from "#imports";

export default defineNuxtRouteMiddleware(async (_to, _from) => {
    const { isAuthenticated, restore } = useAuth();

    if (isAuthenticated.value) {
        return;
    }

    await restore();

    if (!isAuthenticated.value) {
        return navigateTo("/");
    }
});
