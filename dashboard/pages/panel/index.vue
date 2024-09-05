<script async lang="ts" setup>
const { data: players } = await useFetch("/api/players", {
    method: "GET",
    mode: "cors",
    credentials: "include",
    headers: {
        "Content-Type": "application/json",
        ...useRequestHeaders(["cookie"]),
    },
});
</script>

<template>
    <main>
        <ol>
            <li v-for="{ id, name, lastname, number } in players" :key="id">
                <h1>
                    {{ name }}
                    {{ lastname }}
                </h1>
                <p>#{{ number }}</p>
                <p></p>
            </li>
        </ol>
    </main>
</template>
