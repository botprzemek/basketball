import type { UseFetchOptions } from "nuxt/app";

export const useAPI = <T>(
    url: string | (() => string),
    options: UseFetchOptions<T> = {},
) =>
    useFetch(url, {
        ...options,
        $fetch: useNuxtApp().$api,
    });
