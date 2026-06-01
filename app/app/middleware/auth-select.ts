export default defineNuxtRouteMiddleware(async (_to, _from) => {
    const { isLogged, isAuthenticated, logout } = useAuth();

    if (isAuthenticated.value) {
        return navigateTo("/");
    }

    if (isLogged.value) {
        return;
    }

    await logout();
    return navigateTo("/auth/login");
});
