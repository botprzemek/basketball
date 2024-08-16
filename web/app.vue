<script async lang="ts" setup>
const { data: players } = await useFetch("/api/players");
</script>

<template>
  {{ players }}
</template>
