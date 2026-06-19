export const useAuth = () => {
    const { $api } = useNuxtApp();

    const contextOrganizations = useState<Array<ContextOrganization> | null>(
        "auth-organizations",
        () => null,
    );
    const contextCurrent = useState<ContextCurrent | null>("auth-current", () => null);

    const isLogged = computed(() => !!contextOrganizations.value);
    const isAuthenticated = computed(() => !!contextCurrent.value);

    const login = async (data: LoginEvent) => {
        await $api("/auth/login", {
            method: "POST",
            body: data,
        });

        contextOrganizations.value =
            await $api<Array<ContextOrganization>>("/auth/context");

        await navigateTo("/auth/select");
    };

    const select = async (data: ContextSelectEvent) => {
        try {
            await $api("/auth/context/select", {
                method: "POST",
                body: data,
            });

            contextOrganizations.value = null;

            await navigateTo("/");
        } catch {
            await logout();
        }
    };

    const current = async () => {
        try {
            contextCurrent.value = await $api<ContextCurrent>("/auth/context/current");
        } catch {
            contextCurrent.value = null;
        }
    };

    const logout = async () => {
        await navigateTo("/auth/login", { replace: true });

        await $api("/auth/logout", {
            method: "POST",
        });

        clearNuxtData();
        contextOrganizations.value = null;
        contextCurrent.value = null;
    };

    return {
        contextOrganizations,
        contextCurrent,
        isLogged,
        isAuthenticated,
        login,
        select,
        current,
        logout,
    };
};
