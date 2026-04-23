export default defineEventHandler(async (event) => {
  const { apiHost } = useRuntimeConfig();
  const { email, password } = await readBody(event);
  const { headers } = await $fetch.raw<AccountIdentity[]>(`${apiHost}/auth/login`, {
    method: "POST",
    body: JSON.stringify({
      email,
      password,
    }),
  });
  const cookies = headers.getSetCookie();

  if (cookies === null) {
    setResponseStatus(event, 401);
  }

  setHeader(event, "Set-Cookie", cookies);
});
