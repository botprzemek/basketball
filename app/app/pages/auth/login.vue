<script setup lang="ts">
const credentials = ref<LoginCredentials>({
  email: "v@nightcity.net",
  password: "v-1S-th3-Be$t",
});

const auth = useAuth();
</script>

<template>
  <form @submit.prevent="auth.login(credentials)" class="space-y-8">
    <input v-model="credentials.email" type="email" />
    <input v-model="credentials.password" type="password" />
    <button type="submit">Login</button>
  </form>
</template>
