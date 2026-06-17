export default defineNuxtRouteMiddleware(async (_to, _from) => {
    const { $api } = useNuxtApp();
    const { isLogged, isAuthenticated, current } = useAuth();

    if (isLogged.value) {
        return;
    }

    if (!isAuthenticated.value) {
        await current();
    }

    if (isAuthenticated.value) {
        return navigateTo("/");
    }

    await $api("/auth/logout", {
        method: "POST",
    });

    clearNuxtData();
    clearNuxtState(["auth-organizations", "auth-context"]);

    return navigateTo("/auth/login", { replace: true });
});
