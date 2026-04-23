export default defineEventHandler(async (event) => {
  const { apiHost } = useRuntimeConfig();
  const { headers } = await $fetch.raw(`${apiHost}/auth/logout`, {
    method: "POST",
  });

  const cookies = headers.getSetCookie();
  if (cookies === null) {
    setResponseStatus(event, 401);
    return;
  }

  setHeader(event, "Set-Cookie", cookies);
});
