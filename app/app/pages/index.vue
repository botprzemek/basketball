<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const { logout } = useAuth();
</script>

<template>
    <main>
        <aside class="absolute top-0">
            <button @click.prevent="logout()">Logout</button>
        </aside>

        <OrganizationDetails />
        <OrganizationMembers />
        <OrganizationRoles />
    </main>
</template>
