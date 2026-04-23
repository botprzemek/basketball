<script setup lang="ts">
definePageMeta({ middleware: ["auth-access"] });

const auth = useAuth();
const organization = useOrganization();

const details = await organization.getDetails();
const members = await organization.getMembers();
</script>

<template>
  <main>
    <button @click.prevent="auth.logout()">Logout</button>
    {{ details }}
    {{ members }}
  </main>
</template>
