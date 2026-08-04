<script setup lang="ts">
const roles = await useRoles();
</script>

<template>
    <div v-for="role in roles">
        <div class="mb-2">
            <h4>
                {{ role.name }}
            </h4>
            <p>
                {{ role.description }}
            </p>
        </div>

        <div>
            <span>Uuid {{ role.id.substring(0, 8) }}...</span>
        </div>
    </div>
</template>
