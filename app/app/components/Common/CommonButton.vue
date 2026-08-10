<script setup lang="ts">
const state = ref("test");
</script>

<template>
    <button
        :class="[
            `flex w-full px-1.5 py-0.5`,
            `bg-slate-900`,
            `text-white`,
            `hover:cursor-pointer hover:bg-slate-700`,
            // state === `test` ? `hover:cursor-progress` : ``,
        ]"
    >
        <span>[ </span>
        <span :class="`flex-1`">
            <slot />
        </span>
        <span> ]</span>
    </button> 
</template>
