<template>
    <main
        class="relative flex min-h-screen justify-center overflow-hidden bg-[#E8F1F2]"
    >
        <aside
            class="pointer-events-none absolute inset-0 z-0 opacity-60"
            style="
                background-image:
                    linear-gradient(to right, #cedee2 1px, transparent 1px),
                    linear-gradient(to bottom, #cedee2 1px, transparent 1px);
                background-size: 24px 24px;
            "
        ></aside>

        <aside
            class="pointer-events-none absolute inset-0 z-0"
            style="
                background-image:
                    linear-gradient(to right, #b8d4db 1px, transparent 1px),
                    linear-gradient(to bottom, #b8d4db 1px, transparent 1px);
                background-size: 120px 120px;
            "
        ></aside>

        <main class="z-1">
            <slot />
        </main>
    </main>
</template>
