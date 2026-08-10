<template>
    <NuxtLink
        :class="[
            `flex w-full px-3.5 py-2.5`,
            `bg-slate-900`,
            `text-white`,
            `hover:cursor-pointer hover:bg-slate-700`,
        ]"
    >
        <slot />
    </NuxtLink>
</template>
