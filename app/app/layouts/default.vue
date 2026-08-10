<template>
    <aside
        class="pointer-events-none fixed inset-0 z-0 m-6 bg-center opacity-20"
        style="
            background-image:
                linear-gradient(
                    to right,
                    var(--color-gray-400) 1px,
                    transparent 1px
                ),
                linear-gradient(
                    to bottom,
                    var(--color-gray-400) 1px,
                    transparent 1px
                );
            background-size: calc(var(--spacing) * 14) calc(var(--spacing) * 14);
        "
    ></aside>

    <aside
        class="pointer-events-none fixed inset-0 z-0 m-6 border border-slate-400 bg-center opacity-30"
        style="
            background-image:
                linear-gradient(
                    to right,
                    var(--color-slate-400) 1px,
                    transparent 1px
                ),
                linear-gradient(
                    to bottom,
                    var(--color-slate-400) 1px,
                    transparent 1px
                );
            background-size: calc(var(--spacing) * 42) calc(var(--spacing) * 42);
        "
    ></aside>

    <aside class="fixed right-0 bottom-0 z-100">
        <CommonLink
            to="https://github.com/botprzemek/auth"
            target="_blank"
            external
        >
            <IconGithub class="h-auto w-20" />
        </CommonLink>
    </aside>

    <main
        class="relative z-1 flex w-full flex-col items-center-safe justify-center-safe gap-24"
    >
        <slot />
    </main>
</template>
