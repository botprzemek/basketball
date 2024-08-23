<script lang="ts" setup>
const credentials = ref<{ email: string; password: string }>({
    email: "name@example.com",
    password: "S3cuRePa$$w0rd"
});
</script>

<template>
    <h1>Register</h1>
    <form @submit.prevent="useRegister(credentials)">
        <label>E-mail</label>
        <input
            v-model.lazy.trim="credentials.email"
            placeholder="name@example.com"
            type="email"
        />
        <label>Password</label>
        <input
            v-model.lazy.trim="credentials.password"
            placeholder="your-secret-password"
            type="password"
        />
        <button type="submit">Register</button>
    </form>
</template>
