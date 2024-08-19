<script async lang="ts" setup>
const players = ref([]);

const {data: test} = await useFetch("/api/players", {
    method: "GET",
    mode: "cors",
    credentials: "include",
    headers: {
        "Content-Type": "application/json"
    }
});

onMounted(async () => {
    players.value = await $fetch("http://localhost:3001/api/players", {
        method: "GET",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json"
        }
    });
});
</script>

<template>
    <NuxtLink to="/auth/register">Register</NuxtLink>
    <NuxtLink to="/auth/login">Login</NuxtLink>
    {{ players }}
</template>