<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const { organization, roles } = await useOrganization();
</script>

<template>
    <main>
        <h1>{{ organization.slug }}</h1>
        <p>{{ organization.name }}</p>
        <div>
            Roles
            <p v-for="role in roles">{{ role }}</p>
        </div>
    </main>
</template>
