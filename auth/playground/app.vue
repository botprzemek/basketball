<script setup lang="ts">
const { user, isAuthenticated, login, restore, logout } = useAuth();

const handleLogin = async () => {
    await login("v@nightcity.net", "v-1S-th3-Be$t");
};

const handleLogout = async () => {
    await logout();
};

restore();
</script>

<template>
    {{ user }}
    <button @click="handleLogin" v-if="!isAuthenticated">Login</button>
    <button @click="handleLogout" v-else>Logout</button>
</template>
