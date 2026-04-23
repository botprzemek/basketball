<script setup lang="ts">
const credentials = ref<RegisterCredentials>({
  email: "v@nightcity.net",
  password: "v-1S-th3-Be$t",
  firstName: "V",
  lastName: "Venture",
});

const auth = useAuth();
</script>

<template>
  <form @submit.prevent="auth.register(credentials)">
    <input v-model="credentials.email" type="email" />
    <input v-model="credentials.password" type="password" />
    <input v-model="credentials.firstName" type="text" />
    <input v-model="credentials.lastName" type="text" />
    <button type="submit">Register</button>
  </form>
</template>
