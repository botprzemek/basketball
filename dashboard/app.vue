<script lang="ts" setup>
console.log(useRequestHeaders(["cookie"]).cookie);
console.log(useCookie("access-token").value);
console.log(useCookie("refresh-token").value);
</script>

<template>
    <NuxtPage />
</template>
