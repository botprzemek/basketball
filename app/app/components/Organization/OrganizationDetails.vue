<script setup lang="ts">
const organization = await useOrganization();
</script>

<template>
    <div v-if="organization">
        <div>
            <div>
                <span>Organization</span>
                <h2>
                    {{ organization.name }}
                </h2>
            </div>
            <span> @{{ organization.slug }} </span>
        </div>

        <div>
            <div>
                <span>Uuid</span>
                <span>{{ organization.id }}</span>
            </div>
            <div>
                <span>Created</span>
                <span>{{
                    new Date(organization.createdAt).toLocaleDateString()
                }}</span>
            </div>
        </div>
    </div>
</template>
