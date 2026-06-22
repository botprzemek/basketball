<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });
</script>

<template>
    <main>
        <OrganizationDetails />
        <OrganizationMembers />
        <!-- <OrganizationGroups/> -->
        <OrganizationRoles />
    </main>
</template>
