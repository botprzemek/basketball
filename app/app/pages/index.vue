<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const organization = await useOrganization();
const identity = await useIdentity();
const roles = await useRoles();
</script>

<template>
    <main>
        Organization
        <h1>{{ organization }}</h1>
        <p>{{ identity }}</p>
        <div>
            Roles
            <p v-for="role in roles">{{ role }}</p>
        </div>
    </main>
</template>
