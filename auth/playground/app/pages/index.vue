<script setup lang="ts">
interface Credentials {
    email: string;
    password: string;
}

const { login } = useAuth();

const credentials = ref<Credentials>({
    email: "v@nightcity.net",
    password: "v-1S-th3-Be$t",
});

const handleLogin = async ({ email, password }: Credentials) => {
    await login(email, password);
};
</script>

<template>
    <button @click="handleLogin(credentials)">Login</button>
</template>
