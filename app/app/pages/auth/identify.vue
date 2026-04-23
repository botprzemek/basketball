<script setup lang="ts">
definePageMeta({
  middleware: ["auth-identity"],
});

const auth = useAuth();
const identities = await auth.getIdentities();
</script>

<template>
  <main>
    <div v-for="identity in identities" :key="identity.identityId">
      <button @click.prevent="auth.identify(identity.identityId)">
        {{ identity.organizationSlug }}
        {{ identity.organizationName }}
        {{ identity.createdAt }}
      </button>
    </div>
  </main>
</template>
