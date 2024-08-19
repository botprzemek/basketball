<script async lang="ts" setup>
const credentials = ref<{ email: string; password: string }>({
    email: "",
    password: "",
});

const send = async () => {
    const request = await $fetch("/api/auth/login", {
        method: "POST",
        mode: "cors",
        body: JSON.stringify(credentials.value),
    });

    console.log(request);
}
</script>


<template>
    <h1>Login</h1>
    <form @submit.prevent="send()">
        <label>E-mail</label>
        <input v-model.lazy.trim="credentials.email" placeholder="name@example.com" type="email"/>
        <label>Password</label>
        <input v-model.lazy.trim="credentials.password" placeholder="your-secret-password" type="password"/>
        <button type="submit">Login</button>
    </form>
</template>