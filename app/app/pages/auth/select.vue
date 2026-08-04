<script setup lang="ts">
definePageMeta({ middleware: ["auth-select"] });

const { contextOrganizations, select } = useAuth();
</script>

<template>
    <div>
        <div>
            <span>Context</span>
            <h2>Choose a organization</h2>
        </div>

        <form>
            <ul>
                <li
                    v-for="context in contextOrganizations"
                    :key="context.organization.id"
                >
                    <button @click.prevent="select(context)">
                        <div>
                            <span>
                                {{ context.organization.name }}
                            </span>
                            <div>
                                Agent -
                                <span>{{ context.member.name }}</span>
                            </div>
                        </div>

                        <div>
                            <span> @{{ context.organization.slug }} </span>
                        </div>
                    </button>
                </li>
            </ul>
        </form>
    </div>
</template>
