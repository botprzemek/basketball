export const useAuth = () => {
    const { $api } = useNuxtApp();
    const organizations = useState<any | null>(
        "auth-organizations",
        () => null,
    );
    const context = useState<any | null>("auth-context", () => null);

    const isLogged = computed(() => !!organizations.value);
    const isAuthenticated = computed(() => !!context.value);

    const login = async (credentials: LoginCredentials) => {
        await $api("/auth/login", {
            method: "POST",
            body: credentials,
        });

        organizations.value = await $api("/auth/context");

        await navigateTo("/auth/select");
    };

    const select = async (organization: any) => {
        await $api("/auth/context/select", {
            method: "POST",
            body: {
                organizationId: organization.id,
            },
        });

        await navigateTo("/");
    };

    const current = async () => {
        try {
            context.value = await $api("/auth/context/current");
        } catch {
            context.value = null;
        }
    };

    const logout = async () => {
        await navigateTo("/auth/login", { replace: true });

        await $api("/auth/logout", {
            method: "POST",
        });

        clearNuxtData();
        context.value = null;
        organizations.value = null;
    };

    return {
        organizations,
        context,
        isLogged,
        isAuthenticated,
        login,
        select,
        current,
        logout,
    };
};
