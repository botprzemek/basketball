export default defineEventHandler(async (event) => {
  const accessToken = getCookie(event, "marketing-api-access-token");
  if (accessToken !== undefined) {
    await $fetch("/api/auth/logout", {
      method: "POST",
    });

    return;
  }

  const refreshToken = getCookie(event, "marketing-api-refresh-token");
  if (refreshToken === undefined) {
    setResponseStatus(event, 401);
    return;
  }

  const { headers } = await $fetch.raw("http://localhost:8787/v1/auth/refresh", {
    method: "POST",
    headers: {
      Cookie: `marketing-api-refresh-token=${refreshToken}`,
    },
  });

  const cookies = headers.getSetCookie();
  if (cookies === null) {
    setResponseStatus(event, 401);
    return;
  }

  setHeader(event, "Set-Cookie", cookies);
});
