export default defineEventHandler(async (event) => {
  const { email, password, firstName, lastName } = await readBody(event);
  const { _data } = await $fetch.raw("http://localhost:8787/v1/auth/register", {
    method: "POST",
    body: {
      email,
      password,
      firstName,
      lastName,
    },
  });

  return _data;
});
