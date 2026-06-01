<script setup lang="ts">
const { isLogged, isAuthenticated, logout } = useAuth();
</script>

<template>
    <NuxtLayout>
        <aside class="absolute top-0">
            <p>isLogged: {{ isLogged }}</p>
            <p>isAuthenticated: {{ isAuthenticated }}</p>
            <button @click.prevent="logout()">Logout</button>
        </aside>
        <NuxtPage />
    </NuxtLayout>
</template>
