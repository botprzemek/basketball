<script setup lang="ts">
definePageMeta({
    middleware: "auth",
});

const { session, isAuthenticated, logout } = useAuth();

const handleLogout = async () => {
    await logout();
};

const handleTest = async () => {
    await $fetch("http://localhost:3000/api/v1/auth/me", {
        method: "GET",
        credentials: "include",
    });
};
</script>

<template>
    <button @click="handleLogout()">Logout</button>
    <p>
        {{ session }}
        <button @click="handleTest()">Test</button>
    </p>
</template>
