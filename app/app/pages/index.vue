<script setup lang="ts">
definePageMeta({ middleware: ["auth-context"] });

const { logout } = useAuth();

const organization = await useOrganization();
const members = await useMembers();
const roles = await useRoles();
</script>

<template>
    <main>
        <aside class="absolute top-0">
            <button @click.prevent="logout()">Logout</button>
        </aside>

        Organization
        <h1>{{ organization }}</h1>

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
