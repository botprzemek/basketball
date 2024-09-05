<script async lang="ts" setup>
const validators = {
    email: /^[\w-.]{2,32}@([\w-]{2,32}.)+[\w-]{2,4}$/,
    password: /^(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*\W)(?!.* ).{8,16}$/,
};

const credentials = ref<{ email: string; password: string }>({
    email: "test@test.com",
    password: "Test123",
});
const status = ref<string>("Unsent");

const send = async () => {
    const valid = ref<boolean>(false);

    Object.keys(credentials.value).forEach((credential) => {
        valid.value = validators[credential as keyof typeof validators].test(
            credentials.value[credential as keyof typeof validators],
        );
    });

    if (!valid.value) {
        return;
    }

    const data = await $fetch("/api/auth/register", {
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(credentials.value),
    });

    status.value = data.error ? data.error.message : data.message;
};
</script>

<template>
    <h1>Register</h1>
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
        <button type="submit">Register</button>
    </form>
    <p>{{ status }}</p>
</template>
