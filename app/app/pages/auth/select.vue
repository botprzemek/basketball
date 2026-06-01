<script setup lang="ts">
definePageMeta({ middleware: ["auth-select"] });

const { organizations, select, logout } = useAuth();
</script>

<template>
    <form>
        <button @click="logout()">Logout</button>

        <ul>
            <li
                v-for="{ organization, joinedAt } in organizations"
                :key="organization.id"
            >
                <button @click.prevent="select(organization)">
                    [{{ organization.name }}] -
                    {{ new Date(joinedAt).toLocaleDateString("pl-PL") }}
                </button>
            </li>
        </ul>
    </form>
</template>
