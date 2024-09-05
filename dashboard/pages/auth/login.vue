<script async lang="ts" setup>
const credentials = ref<{ email: string; password: string }>({
    email: "test@test.com",
    password: "Test123",
});

const send = async () => {
    const data = await $fetch("/api/auth/login", {
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(credentials.value),
    });

    if (!data) {
        return;
    }

    navigateTo({ path: "/panel" });
};
</script>

<template>
    <h1>Login</h1>
    <form @submit.prevent="send()">
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
        <button type="submit">Login</button>
    </form>
</template>
