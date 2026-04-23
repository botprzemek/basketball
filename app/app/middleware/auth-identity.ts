export default defineNuxtRouteMiddleware(async (_to, _from) => {
  if (import.meta.client) {
    return;
  }

  const { tokens } = useRuntimeConfig();
  const token = useCookie(tokens.identity);
  if (token.value !== undefined) {
    return;
  }

  console.log("logging the fuck out");

  await $fetch.raw("/api/auth/logout", {
    method: "POST",
  });

  return navigateTo("/auth/login");
});
