export default defineNuxtRouteMiddleware(async (to, from) => {
    if (to.path.includes("/auth")) {
        return;
    }

    console.log(useCookie("refresh-token").value);

    // if (!useCookie("refresh-token").value) {
    //     return navigateTo({ path: "/auth/login" });
    // }
    // const tokens = useCookie("access-token").value;
    // console.log(tokens);
});
