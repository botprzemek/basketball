<script setup lang="ts">
const groups = await useRoles();
</script>

<template>
    <div v-for="group in groups">
        <div class="mb-2">
            <h4>
                {{ group.name }}
            </h4>
            <p>
                {{ group.description }}
            </p>
        </div>

        <div>
            <span>Uuid {{ group.id.substring(0, 8) }}...</span>
        </div>
    </div>
</template>
