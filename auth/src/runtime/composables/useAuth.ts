import {
    clearNuxtState,
    computed,
    useAPI,
    useNuxtApp,
    useRequestHeaders,
    useState,
} from "#imports";

export const useAuth = () => {
    const { $api, callHook } = useNuxtApp();
    const session = useState<any | null>("auth_session", () => null);

    const isAuthenticated = computed(() => !!session.value);

    const login = async (email: string, password: string) => {
        await $api("/auth/login", {
            method: "POST",
            credentials: "include",
            body: { email, password },
        });

        session.value = await $api("/auth/me", {
            method: "GET",
            credentials: "include",
        });

        await callHook("auth:login", "/auth");
    };

    const restore = async () => {
        const { data } = await useAPI("/auth/me", {
            method: "GET",
            credentials: "include",
            headers: useRequestHeaders(["cookie"]),
        });

        session.value = data.value;
    };

    const logout = async () => {
        await $api("/auth/logout", {
            method: "POST",
            credentials: "include",
        });

        clearNuxtState("auth_session");

        await callHook("auth:logout", "/");
    };

    return {
        session,
        isAuthenticated,
        login,
        restore,
        logout,
    };
};
