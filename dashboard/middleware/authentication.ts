export default defineNuxtRouteMiddleware(async () => {
    if (useCookie("access-token").value) {
        return;
    }

    return navigateTo("/auth/login");
});
