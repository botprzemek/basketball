export default defineEventHandler(async (event) => {
  const { apiHost, tokens } = useRuntimeConfig();
  const token = getCookie(event, tokens.identity);
  if (token === undefined) {
    setResponseStatus(event, 401);
    return;
  }

  const { identityId } = await readBody(event);
  const { headers } = await $fetch.raw(`${apiHost}/auth/identify`, {
    method: "POST",
    headers: {
      Cookie: `${tokens.identity}=${token}`,
    },
    body: {
      identityId,
    },
  });

  const cookies = headers.getSetCookie();
  if (cookies === null) {
    setResponseStatus(event, 401);
    return;
  }

  setHeader(event, "Set-Cookie", cookies);
});
