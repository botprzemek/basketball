export default defineNuxtRouteMiddleware(async (_to, _from) => {
    const { $api } = useNuxtApp();
    const { isAuthenticated, current } = useAuth();

    if (!isAuthenticated.value) {
        await current();
    }

    if (isAuthenticated.value) {
        return;
    }

    await $api("/auth/logout", {
        method: "POST",
    });

    clearNuxtData();
    clearNuxtState(["auth-organizations", "auth-context"]);

    return navigateTo("/auth/login", { replace: true });
});
