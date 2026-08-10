<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const { logout } = useAuth();
</script>

<template>
    <main>
        <CommonButton @click="logout"> Logout </CommonButton>
        <OrganizationDetails />
        <OrganizationMembers />
        <OrganizationGroups />
        <OrganizationRoles />
    </main>
</template>
