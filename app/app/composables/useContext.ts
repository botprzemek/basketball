import type { UseFetchOptions } from "nuxt/app";

export const useContext = <T>(
    url: string | (() => string),
    options: UseFetchOptions<T> = {},
) => {
    const { context } = useAuth();

    return useFetch(`/organizations/${context.value.organizationId}${url}`, {
        ...options,
        $fetch: useNuxtApp().$api,
    });
};
