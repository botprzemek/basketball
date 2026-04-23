export const useAuth = () => {
  const getIdentities = async () => {
    const { data: identities } = await useFetch("/api/auth/identities", {
      key: "auth-identities",
    });

    return identities;
  };

  const register = async (credentials: RegisterCredentials) => {
    await $fetch("/api/auth/register", {
      method: "POST",
      body: credentials,
    });

    return navigateTo("/auth/login");
  };

  const login = async (credentials: LoginCredentials) => {
    await $fetch("/api/auth/login", {
      method: "POST",
      body: credentials,
    });

    return navigateTo("/auth/identify");
  };

  const identify = async (identityId: string) => {
    await $fetch("/api/auth/identify", {
      method: "POST",
      body: { identityId },
    });

    clearNuxtData("auth-identities");

    return navigateTo("/");
  };

  const logout = async () => {
    await $fetch("/api/auth/logout", {
      method: "POST",
    });

    clearNuxtData("auth-identities");

    return navigateTo("/auth/login");
  };

  return {
    getIdentities,
    register,
    login,
    identify,
    logout,
  };
};
