<script setup lang="ts">
const { login } = useAuth();

const credentials = ref({
    email: "v@nightcity.net",
    password: "v-1S-th3-Be$t",
});
</script>

<template>
    <div>
        <div>
            <span>Auth</span>
            <h2>Log into the organization</h2>
        </div>

        <form @submit.prevent="login(credentials)">
            <div>
                <label>E-mail address</label>
                <input
                    v-model="credentials.email"
                    type="email"
                    placeholder="v@nightcity.net"
                />
            </div>

            <div>
                <label>Password</label>
                <input
                    v-model="credentials.password"
                    type="password"
                    placeholder="********"
                />
            </div>

            <button type="submit">Authenticate</button>
        </form>
    </div>
</template>
