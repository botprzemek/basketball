import {
    clearNuxtState,
    computed,
    useFetch,
    useNuxtApp,
    useState,
} from "#imports";

export const useAuth = () => {
    const nuxtApp = useNuxtApp();
    const session = useState<any | null>("auth_session", () => null);

    const isAuthenticated = computed(() => !!session.value);

    const login = async (email: string, password: string) => {
        try {
            const data = await $fetch("/api/auth/login", {
                method: "POST",
                body: { email, password },
            });

            session.value = data;

            await nuxtApp.callHook("auth:login", "/auth");
        } catch (error) {
            console.error("Login failed", error);
            return { success: false, error };
        }
    };

    const restore = async () => {
        const { data } = await useFetch("/api/auth/me", {
            method: "GET",
        });

        session.value = data.value;
    };

    const logout = async () => {
        await $fetch("/api/auth/logout", {
            method: "POST",
        });

        clearNuxtState("auth_session");

        await nuxtApp.callHook("auth:logout", "/");
    };

    return {
        session,
        isAuthenticated,
        login,
        restore,
        logout,
    };
};
