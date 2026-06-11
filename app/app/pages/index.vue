<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const organization = await useOrganization();
const identity = await useIdentity();
const members = await useMembers();
const roles = await useRoles();
</script>

<template>
    <main>
        Organization
        <h1>{{ organization }}</h1>
        <p>{{ identity }}</p>
        <div>
            Members
            <p v-for="member in members">{{ member }}</p>
        </div>
        <div>
            Roles
            <p v-for="role in roles">{{ role }}</p>
        </div>
    </main>
</template>
