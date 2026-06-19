<script setup lang="ts">
const { login } = useAuth();

const credentials = ref({
    email: "v@nightcity.net",
    password: "v-1S-th3-Be$t",
});
</script>

<template>
    <div
        class="mx-auto w-full max-w-md rounded-xl border border-slate-800 bg-slate-900 p-6 text-slate-100 shadow-lg"
    >
        <div class="mb-6">
            <span
                class="font-mono text-xs font-semibold tracking-wider text-red-500 uppercase"
                >Terminal Uwierzytelniania</span
            >
            <h2 class="mt-1 text-2xl font-bold tracking-tight text-white">
                Zaloguj się do sieci
            </h2>
        </div>

        <form @submit.prevent="login(credentials)" class="flex flex-col gap-4">
            <div>
                <label
                    class="mb-1 block font-mono text-[10px] tracking-wider text-slate-500 uppercase"
                    >Adres Email / Login</label
                >
                <input
                    v-model="credentials.email"
                    type="email"
                    placeholder="v@nightcity.net"
                    class="w-full rounded-lg border border-slate-800 bg-slate-950 px-3 py-2 font-mono text-sm text-slate-100 transition-colors outline-none placeholder:text-slate-700 focus:border-red-500/50 focus:ring-1 focus:ring-red-500/50"
                />
            </div>

            <div>
                <label
                    class="mb-1 block font-mono text-[10px] tracking-wider text-slate-500 uppercase"
                    >Klucz Dostępu (Hasło)</label
                >
                <input
                    v-model="credentials.password"
                    type="password"
                    placeholder="********"
                    class="w-full rounded-lg border border-slate-800 bg-slate-950 px-3 py-2 font-mono text-sm text-slate-100 transition-colors outline-none placeholder:text-slate-700 focus:border-red-500/50 focus:ring-1 focus:ring-red-500/50"
                />
            </div>

            <button
                type="submit"
                class="mt-2 rounded-lg border border-red-500/20 bg-red-600/10 py-2.5 font-mono text-sm font-bold tracking-widest text-red-400 uppercase shadow-md transition-all hover:border-red-600 hover:bg-red-600 hover:text-white active:scale-[0.98]"
            >
                Autoryzuj Połączenie
            </button>
        </form>
    </div>
</template>
