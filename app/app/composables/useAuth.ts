export const useAuth = () => {
    const { $api, callHook } = useNuxtApp();
    const organizations = useState<any | null>(
        "auth_organizations",
        () => null,
    );
    const context = useState<any | null>("auth_current", () => null);

    const isLogged = computed(() => !!organizations.value);
    const isAuthenticated = computed(() => !!context.value);

    const login = async (credentials: LoginCredentials) => {
        await $api("/auth/login", {
            method: "POST",
            body: credentials,
        });

        organizations.value = await $api("/auth/context", {
            method: "GET",
        });

        await callHook("auth:login");
    };

    const select = async (organization: any) => {
        await $api("/auth/context/select", {
            method: "POST",
            body: {
                organizationId: organization.id,
            },
        });

        await callHook("auth:select");
    };

    const current = async () => {
        try {
            context.value = await $api("/auth/context/current");
        } catch {
            context.value = null;
        }
    };

    const logout = async () => {
        await callHook("auth:logout");

        await $api("/auth/logout", {
            method: "POST",
        });
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
