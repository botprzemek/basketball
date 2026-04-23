export default defineEventHandler(async (event) => {
  const { apiHost, tokens } = useRuntimeConfig();
  const { _data } = await $fetch.raw<Array<AccountIdentity>>(`${apiHost}/auth/identities`, {
    method: "GET",
    headers: {
      Cookie: `${tokens.identity}=${getCookie(event, tokens.identity)}`,
    },
  });

  return _data;
});
