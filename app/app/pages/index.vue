<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const organization = await useOrganization();
const roles = await useRoles();
</script>

<template>
    <main>
        Organization
        <h1>{{ organization.name }}</h1>
        <p>[ {{ organization.slug }} ]</p>
        <div>
            Roles
            <p v-for="role in roles">{{ role }}</p>
        </div>
    </main>
</template>
