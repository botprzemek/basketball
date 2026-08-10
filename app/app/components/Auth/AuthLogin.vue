<script setup lang="ts">
const showPopup = ref(false);
const countdown = ref(3);

const data = ref({
    email: "v@nightcity.net",
    password: "v-1S-th3-Be$t",
});

const handleLogin = async () => {
    // Symulacja logowania
    // await useAuth().login(data.value);
    showPopup.value = true;

    const timer = setInterval(() => {
        countdown.value--;
        if (countdown.value <= 0) {
            clearInterval(timer);
            navigateTo("/auth/identify");
        }
    }, 1000);
};

// --- LOGIKA CANVAS (INTERAKTYWNY KURSOR) ---
const canvasRef = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;
const gridSize = 24;
const chars = ["+", "×", "□", "•", "·", "/", "—", "0", "1"];

const mouse = reactive({ x: -1000, y: -1000 });
const radius = 80;

interface CharObject {
    char: string;
    baseX: number;
    baseY: number;
    x: number;
    y: number;
}

let particles: CharObject[] = [];

const initParticles = () => {
    if (!canvasRef.value) return;
    particles = [];
    const { width, height } = canvasRef.value;

    for (let x = 0; x < width; x += gridSize) {
        for (let y = 0; y < height; y += gridSize) {
            const char = chars[Math.floor(Math.random() * chars.length)];
            particles.push({
                char,
                baseX: x + 4,
                baseY: y + 12,
                x: x + 4,
                y: y + 12,
            });
        }
    }
};

const handleMouseMove = (e: MouseEvent) => {
    if (!canvasRef.value) return;
    const rect = canvasRef.value.getBoundingClientRect();
    mouse.x = e.clientX - rect.left;
    mouse.y = e.clientY - rect.top;
};

const animate = () => {
    if (!ctx || !canvasRef.value) return;
    const { width, height } = canvasRef.value;

    ctx.fillStyle = "#f8fafc";
    ctx.fillRect(0, 0, width, height);

    particles.forEach((p) => {
        const dx = mouse.x - p.baseX;
        const dy = mouse.y - p.baseY;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < radius) {
            const force = (radius - distance) / radius;
            const moveX = (dx / distance) * force * 15;
            const moveY = (dy / distance) * force * 15;

            p.x = p.baseX - moveX;
            p.y = p.baseY - moveY;
            ctx.fillStyle = "#0f172a";
        } else {
            p.x += (p.baseX - p.x) * 0.1;
            p.y += (p.baseY - p.y) * 0.1;
            ctx.fillStyle = "#cbd5e1";
        }

        ctx.font = "18px Space Mono";
        ctx.fillText(p.char, p.x, p.y);
    });

    requestAnimationFrame(animate);
};

onMounted(() => {
    if (canvasRef.value) {
        ctx = canvasRef.value.getContext("2d");
        const rect = canvasRef.value.getBoundingClientRect();
        canvasRef.value.width = rect.width;
        canvasRef.value.height = rect.height;

        initParticles();
        animate();
    }
});
</script>

<template>
    <main
        class="relative flex min-h-screen items-center justify-center overflow-hidden bg-[#E8F1F2] p-6"
        style="
            background-image:
                linear-gradient(#cedee2 1px, transparent 1px),
                linear-gradient(90deg, #cedee2 1px, transparent 1px);
            background-size: 40px 40px;
        "
    >
        <div
            class="relative flex h-[650px] w-full max-w-5xl flex-col overflow-hidden border border-slate-300 bg-white/95 shadow-2xl backdrop-blur-md md:flex-row"
        >
            <div
                class="relative z-10 flex w-full flex-col justify-center border-r border-slate-100 bg-white p-12 md:w-1/2 lg:p-16"
            >
                <header class="mb-12">
                    <div class="mb-4 flex items-center gap-2">
                        <div class="h-3 w-1 bg-slate-900"></div>
                        <span
                            class="text-[9px] font-black tracking-[0.4em] text-slate-400 uppercase"
                            >Core_Identity // B17X</span
                        >
                    </div>

                    <h1
                        data-text="SYSTEM_ACCESS"
                        class="relative text-6xl leading-none font-black tracking-tighter text-slate-900 after:absolute after:top-[3px] after:left-[3px] after:-z-10 after:text-slate-100 after:content-[attr(data-text)]"
                    >
                        SYSTEM_ACCESS
                    </h1>
                </header>

                <form @submit.prevent="handleLogin" class="space-y-8">
                    <div>
                        <label
                            class="mb-2 block text-[10px] font-bold tracking-widest text-slate-500 uppercase"
                            >Terminal_ID</label
                        >
                        <input
                            v-model="data.email"
                            type="email"
                            class="w-full border border-slate-200 bg-slate-50 p-4 text-base transition-all focus:border-slate-900 focus:outline-none"
                            placeholder="v@nightcity.net"
                        />
                    </div>

                    <div>
                        <label
                            class="mb-2 block text-[10px] font-bold tracking-widest text-slate-500 uppercase"
                            >Access_Phrase</label
                        >
                        <input
                            v-model="data.password"
                            type="password"
                            class="w-full border border-slate-200 bg-slate-50 p-4 text-base transition-all focus:border-slate-900 focus:outline-none"
                            placeholder="••••••••"
                        />
                    </div>

                    <button
                        type="submit"
                        class="group flex w-full items-center justify-between bg-slate-900 px-8 py-5 text-white transition-all hover:bg-black"
                    >
                        <span
                            class="text-[10px] font-bold tracking-[0.3em] uppercase"
                            >Authorize_Session</span
                        >
                        <span
                            class="text-xl transition-transform group-hover:translate-x-1"
                            >→</span
                        >
                    </button>
                </form>

                <footer
                    class="mt-12 flex items-end justify-between border-t border-slate-50 pt-6"
                >
                    <div
                        class="text-[9px] leading-tight font-bold text-slate-400 uppercase"
                    >
                        PROPRIETARY CAPSULE<br />
                        ZERO IMMUNE EXPOSURE
                    </div>
                    <div class="flex gap-1">
                        <div class="h-1.5 w-1.5 bg-slate-900"></div>
                        <div class="h-1.5 w-1.5 bg-slate-100"></div>
                    </div>
                </footer>
            </div>

            <div
                class="relative hidden overflow-hidden bg-slate-50 md:block md:w-1/2"
            >
                <canvas
                    ref="canvasRef"
                    class="h-full w-full opacity-70"
                    @mousemove="handleMouseMove"
                    @mouseleave="
                        mouse.x = -1000;
                        mouse.y = -1000;
                    "
                ></canvas>

                <div
                    class="pointer-events-none absolute top-8 right-8 text-right"
                >
                    <div class="mb-1 flex items-center justify-end gap-2">
                        <span
                            class="text-[10px] font-black tracking-tighter text-slate-900 uppercase"
                            >Status: Optimal</span
                        >
                        <div class="h-2 w-2 animate-pulse bg-slate-900"></div>
                    </div>
                    <div
                        class="font-mono text-[8px] tracking-widest text-slate-400"
                    >
                        ENCRYPTED_STREAM_X4
                    </div>
                </div>

                <div
                    class="pointer-events-none absolute bottom-0 left-0 w-full p-8"
                >
                    <div
                        class="relative border border-slate-200 bg-white p-6 shadow-xl"
                    >
                        <div
                            class="absolute -top-3 left-4 bg-black px-2 py-0.5 text-[8px] font-bold tracking-widest text-white uppercase"
                        >
                            Target_Data_Node
                        </div>
                        <div class="flex items-end justify-between">
                            <div>
                                <h3
                                    class="text-3xl font-black tracking-tighter text-slate-900 italic"
                                >
                                    B17X_SYNT_B3
                                </h3>
                                <p
                                    class="mt-1 text-[9px] font-bold tracking-widest text-slate-500 uppercase"
                                >
                                    Vector-Cell Delivery Unit-CT
                                </p>
                            </div>
                            <div
                                class="text-right font-mono text-[9px] text-slate-400"
                            >
                                100% - ACCURACY<br />
                                POINT ST: 98MM
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <Transition name="fade">
                <div
                    v-if="showPopup"
                    class="absolute inset-0 z-50 flex items-center justify-center bg-slate-900/10 p-6 backdrop-blur-sm"
                >
                    <div
                        class="relative w-full max-w-md border-2 border-slate-900 bg-white shadow-[16px_16px_0px_0px_rgba(15,23,42,0.1)]"
                    >
                        <div
                            class="flex items-center justify-between bg-slate-900 p-4 text-white"
                        >
                            <span
                                class="text-[9px] font-black tracking-[0.4em] uppercase"
                                >Security_Protocol_Success</span
                            >
                            <div class="bg-white/20 px-2 py-1 text-[9px]">
                                RED_X4
                            </div>
                        </div>

                        <div class="p-10 text-center">
                            <div class="mb-6 text-5xl font-black">✓</div>
                            <h2
                                class="mb-2 text-2xl font-black tracking-tighter text-slate-900"
                            >
                                ACCESS_GRANTED
                            </h2>
                            <p
                                class="mb-8 text-[10px] tracking-widest text-slate-500 uppercase"
                            >
                                Establishing link to identity terminal...
                            </p>

                            <div
                                class="w-full border border-slate-900 bg-slate-50 py-4 text-[10px] font-black tracking-[0.3em] uppercase"
                            >
                                Redirecting ({{ countdown }}s)
                            </div>
                        </div>

                        <div class="flex justify-center gap-1 bg-slate-50 p-3">
                            <div
                                v-for="i in 5"
                                :key="i"
                                class="h-1.5 w-1.5 border border-slate-300"
                                :class="{
                                    'animate-pulse bg-slate-900':
                                        i <= 4 - countdown,
                                }"
                            ></div>
                        </div>
                    </div>
                </div>
            </Transition>
        </div>
    </main>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

input:-webkit-autofill {
    -webkit-box-shadow: 0 0 0px 1000px white inset;
}
</style>
