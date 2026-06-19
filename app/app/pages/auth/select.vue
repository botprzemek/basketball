<script setup lang="ts">
definePageMeta({ middleware: ["auth-select"] });

const { contextOrganizations, select } = useAuth();
</script>

<template>
    <div
        class="mx-auto w-full max-w-md rounded-xl border border-slate-800 bg-slate-900 p-6 text-slate-100 shadow-lg"
    >
        <div class="mb-5">
            <span
                class="font-mono text-xs font-semibold tracking-wider text-red-500 uppercase"
                >Weryfikacja Tożsamości</span
            >
            <h2 class="mt-1 text-2xl font-bold tracking-tight text-white">
                Wybierz profil operacyjny
            </h2>
            <p class="mt-1 text-xs text-slate-400">
                Wykryto powiązania z wieloma węzłami korporacyjnymi.
            </p>
        </div>

        <form>
            <ul class="space-y-3">
                <li
                    v-for="context in contextOrganizations"
                    :key="context.organization.id"
                    class="group"
                >
                    <button
                        @click.prevent="select(context)"
                        class="flex w-full items-center justify-between rounded-xl border border-slate-800 bg-slate-950 p-4 text-left transition-all group-hover:bg-slate-950/80 hover:border-red-500/40 active:scale-[0.99]"
                    >
                        <div class="space-y-1">
                            <span
                                class="font-mono text-xs font-bold tracking-wide text-white uppercase transition-colors group-hover:text-red-400"
                            >
                                {{ context.organization.name }}
                            </span>
                            <div class="font-mono text-[11px] text-slate-400">
                                Agent:
                                <span class="text-slate-300">{{
                                    context.member.name
                                }}</span>
                            </div>
                        </div>

                        <div class="flex flex-col items-end gap-1 text-right">
                            <span
                                class="rounded bg-slate-900 px-2 py-0.5 font-mono text-[10px] text-slate-500 transition-colors group-hover:text-slate-300"
                            >
                                @{{ context.organization.slug }}
                            </span>
                        </div>
                    </button>
                </li>
            </ul>
        </form>

        <div class="mt-5 border-t border-slate-800/60 pt-4 text-center">
            <a
                href="#"
                class="font-mono text-[11px] text-slate-500 underline transition-colors hover:text-slate-400"
            >
                Rozłącz sesję główną
            </a>
        </div>
    </div>
</template>
