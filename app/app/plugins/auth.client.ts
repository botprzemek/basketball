// const handler = async () => {
//   const { name, fullPath } = useRoute();
//   if (name && name.toString().startsWith("auth")) {
//     return;
//   }

//   if (document.visibilityState === "hidden") {
//     return;
//   }

//   const accessToken = useCookie("marketing-api-access-token");
//   const refreshToken = useCookie("marketing-api-refresh-token");
//   if (accessToken.value !== undefined && refreshToken.value === undefined) {
//     await $fetch.raw("/api/auth/logout", {
//       method: "POST",
//     });

//     return navigateTo("/auth/login");
//   }

//   const { ok, headers } = await $fetch.raw("/api/auth/validate", {
//     method: "POST",
//     headers: {
//       Cookie: `marketing-api-refresh-token=${refreshToken.value}; marketing-api-access-token=${accessToken.value}`,
//     },
//   });
//   if (!ok) {
//     await $fetch.raw("/api/auth/logout", {
//       method: "POST",
//     });

//     return navigateTo("/auth/login");
//   }

//   useResponseHeader("Set-Cookie").value = headers.getSetCookie();

//   return navigateTo({
//     path: "/auth",
//     query: {
//       redirect: fullPath,
//     },
//   });
// };

// export default defineNuxtPlugin(() => {
//   document.addEventListener("visibilitychange", handler);
// });
