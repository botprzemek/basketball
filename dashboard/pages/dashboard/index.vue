<script lang="ts" setup>
const { data: players, error } = await useFetch("/api/players", {
    method: "GET",
    mode: "cors",
    credentials: "include",
    headers: {
        "Content-Type": "application/json",
        ...useRequestHeaders(["cookie"])
    }
});

if (error.value) {
    navigateTo("/auth/login");
}
</script>

<template>
    <main>
        <ol>
            <li v-for="{ id, name, lastname, number, nationality } in players" :key="id">
                <h1>
                    {{ name }}
                    {{ lastname }}
                </h1>
                <p>#{{ number }}</p>
                <p>{{ nationality }}</p>
            </li>
        </ol>
    </main>
</template>
