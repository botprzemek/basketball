export default defineNuxtRouteMiddleware(async (_to, _from) => {
    const { isLogged, isAuthenticated, current, logout } = useAuth();

    if (!isLogged.value && isAuthenticated.value) {
        return;
    }

    await current();

    if (isAuthenticated.value) {
        await useOrganization();
        return;
    }

    await logout();
    return navigateTo("/auth/login");
});
