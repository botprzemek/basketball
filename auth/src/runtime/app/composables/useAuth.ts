import { clearNuxtState, computed, useFetch, useState } from "#imports";

export const useAuth = () => {
    const user = useState<any | null>("auth_user", () => null);

    const isAuthenticated = computed(() => !!user.value);

    const login = async (email: string, password: string) => {
        try {
            const data = await $fetch("/api/auth/login", {
                method: "POST",
                body: { email, password },
            });

            user.value = data;

            return { success: true };
        } catch (error) {
            console.error("Login failed", error);
            return { success: false, error };
        }
    };

    const restore = async () => {
        const { data } = await useFetch("/api/auth/me", {
            method: "GET",
        });

        user.value = data.value;
    };

    const logout = async () => {
        await $fetch("/api/auth/logout", {
            method: "POST",
        });

        clearNuxtState("auth_user");
    };

    return {
        user,
        isAuthenticated,
        login,
        restore,
        logout,
    };
};
