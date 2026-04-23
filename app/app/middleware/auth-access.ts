export default defineNuxtRouteMiddleware(async () => {
  if (import.meta.client) {
    return;
  }

  const { tokens } = useRuntimeConfig();

  const identity = useCookie(tokens.identity);
  if (identity.value !== undefined) {
    await $fetch.raw("/api/auth/logout", {
      method: "POST",
    });

    return navigateTo("/auth/login");
  }

  const access = useCookie(tokens.access);
  if (access.value === undefined) {
    await $fetch.raw("/api/auth/logout", {
      method: "POST",
    });

    return navigateTo("/auth/login");
  }

  const refresh = useCookie(tokens.refresh);
  if (refresh.value === undefined) {
    await $fetch.raw("/api/auth/logout", {
      method: "POST",
    });

    return navigateTo("/auth/login");
  }
});
