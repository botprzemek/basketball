export default defineEventHandler(async (event) => {
  const { apiHost, tokens } = useRuntimeConfig();
  const { _data } = await $fetch.raw<Array<Member>>(`${apiHost}/organization/members`, {
    method: "GET",
    headers: {
      Cookie: `${tokens.access}=${getCookie(event, tokens.access)}`,
    },
  });

  return _data;
});
