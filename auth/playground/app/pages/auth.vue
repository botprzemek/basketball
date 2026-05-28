<script setup lang="ts">
definePageMeta({
    middleware: "auth",
});

const { session, isAuthenticated, logout } = useAuth();

const handleLogout = async () => {
    await logout();
};
</script>

<template>
    <button @click="handleLogout()">Logout</button>
    <p>
        Session({{ isAuthenticated }}):
        {{ session }}
    </p>
</template>
