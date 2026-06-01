<script setup lang="ts">
const { login } = useAuth();
const credentials = ref<LoginCredentials>({
    email: "v@nightcity.net",
    password: "v-1S-th3-Be$t",
});
</script>

<template>
    <form @submit.prevent="login(credentials)" class="flex flex-col">
        <input
            v-model="credentials.email"
            type="email"
            placeholder="v@nightcity.net"
        />

        <input
            v-model="credentials.password"
            type="password"
            placeholder="********"
        />

        <button type="submit">Authorize</button>
    </form>
</template>
