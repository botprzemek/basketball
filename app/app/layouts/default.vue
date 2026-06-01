<template>
    <main
        class="relative flex min-h-screen items-center justify-center overflow-hidden bg-[#E8F1F2] p-6"
    >
        <slot />
    </main>
</template>

<style scoped>
main {
    background-image:
        linear-gradient(#cedee2 1px, transparent 1px),
        linear-gradient(90deg, #cedee2 1px, transparent 1px);
    background-size: 40px 40px;
}
</style>
