import type { UseFetchOptions } from "nuxt/app";

export const useContext = <T>(
    url: string | (() => string),
    options: UseFetchOptions<T> = {},
) => {
    const { contextCurrent } = useAuth();

    return useFetch(
        `/organizations/${contextCurrent.value!.organizationId}${url}`,
        {
            ...options,
            $fetch: useNuxtApp().$api,
        },
    );
};
