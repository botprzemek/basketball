<script setup lang="ts">
const { login } = useAuth();

const credentials = ref({
    email: "v@nightcity.net",
    password: "v-1S-th3-Be$t",
});
</script>

<template>
    <main class="flex w-lg flex-col border border-slate-300 bg-slate-50">
        <aside :class="[`bg-slate-900 px-3 py-2`]">
            <p :class="[`text-xs text-slate-50`]">auth@1.0.0</p>
        </aside>

        <header :class="[`mx-16 my-6 flex flex-col gap-2`]">
            <h1 :class="[`text-2xl text-slate-900`]">login</h1>
            <p :class="[`text-sm text-slate-500`]">
                authenticate to access your organization
            </p>
        </header>

        <form
            :class="[`mx-16 my-6 flex flex-col gap-4`]"
            @submit.prevent="login(credentials)"
        >
            <div class="grid grid-flow-row gap-2">
                <label :class="[`text-xs text-slate-500`]">email</label>
                <input
                    v-model="credentials.email"
                    type="email"
                    placeholder="v@nightcity.net"
                    :class="[
                        `border border-slate-300 px-2 py-1 text-sm text-slate-500`,
                        `focus:border-slate-700 focus:text-slate-500 focus:outline-none`,
                    ]"
                />
            </div>

            <div class="grid grid-flow-row gap-2">
                <label :class="[`text-xs text-slate-500`]">password</label>
                <input
                    v-model="credentials.password"
                    type="password"
                    placeholder="********"
                    :class="[
                        `border border-slate-300 px-2 py-1 text-sm text-slate-500`,
                        `focus:border-slate-500 focus:text-slate-500 focus:outline-none`,
                    ]"
                />
            </div>

            <CommonButton type="submit"> authenticate </CommonButton>
        </form>
    </main>
</template>
