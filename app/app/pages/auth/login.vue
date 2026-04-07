<script setup lang="ts">
const router = useRouter();
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
    class="min-h-screen bg-[#E8F1F2] flex items-center justify-center p-6 relative overflow-hidden"
    style="
      background-image:
        linear-gradient(#cedee2 1px, transparent 1px),
        linear-gradient(90deg, #cedee2 1px, transparent 1px);
      background-size: 40px 40px;
    "
  >
    <div
      class="bg-white/95 backdrop-blur-md w-full max-w-5xl border border-slate-300 shadow-2xl flex flex-col md:flex-row overflow-hidden h-[650px] relative"
    >
      <div
        class="w-full md:w-1/2 p-12 lg:p-16 flex flex-col justify-center border-r border-slate-100 bg-white relative z-10"
      >
        <header class="mb-12">
          <div class="flex items-center gap-2 mb-4">
            <div class="h-3 w-1 bg-slate-900"></div>
            <span class="text-[9px] font-black tracking-[0.4em] text-slate-400 uppercase"
              >Core_Identity // B17X</span
            >
          </div>

          <h1
            data-text="SYSTEM_ACCESS"
            class="relative text-6xl font-black tracking-tighter text-slate-900 leading-none after:content-[attr(data-text)] after:absolute after:top-[3px] after:left-[3px] after:-z-10 after:text-slate-100"
          >
            SYSTEM_ACCESS
          </h1>
        </header>

        <form @submit.prevent="handleLogin" class="space-y-8">
          <div>
            <label class="text-[10px] font-bold uppercase tracking-widest text-slate-500 block mb-2"
              >Terminal_ID</label
            >
            <input
              v-model="data.email"
              type="email"
              class="w-full bg-slate-50 border border-slate-200 p-4 focus:outline-none focus:border-slate-900 transition-all text-base"
              placeholder="v@nightcity.net"
            />
          </div>

          <div>
            <label class="text-[10px] font-bold uppercase tracking-widest text-slate-500 block mb-2"
              >Access_Phrase</label
            >
            <input
              v-model="data.password"
              type="password"
              class="w-full bg-slate-50 border border-slate-200 p-4 focus:outline-none focus:border-slate-900 transition-all text-base"
              placeholder="••••••••"
            />
          </div>

          <button
            type="submit"
            class="w-full bg-slate-900 text-white py-5 flex justify-between items-center px-8 hover:bg-black transition-all group"
          >
            <span class="text-[10px] font-bold uppercase tracking-[0.3em]">Authorize_Session</span>
            <span class="text-xl group-hover:translate-x-1 transition-transform">→</span>
          </button>
        </form>

        <footer class="mt-12 pt-6 border-t border-slate-50 flex justify-between items-end">
          <div class="text-[9px] text-slate-400 font-bold uppercase leading-tight">
            PROPRIETARY CAPSULE<br />
            ZERO IMMUNE EXPOSURE
          </div>
          <div class="flex gap-1">
            <div class="w-1.5 h-1.5 bg-slate-900"></div>
            <div class="w-1.5 h-1.5 bg-slate-100"></div>
          </div>
        </footer>
      </div>

      <div class="hidden md:block md:w-1/2 relative bg-slate-50 overflow-hidden">
        <canvas
          ref="canvasRef"
          class="w-full h-full opacity-70"
          @mousemove="handleMouseMove"
          @mouseleave="
            mouse.x = -1000;
            mouse.y = -1000;
          "
        ></canvas>

        <div class="absolute top-8 right-8 text-right pointer-events-none">
          <div class="flex items-center justify-end gap-2 mb-1">
            <span class="text-[10px] font-black text-slate-900 tracking-tighter uppercase"
              >Status: Optimal</span
            >
            <div class="w-2 h-2 bg-slate-900 animate-pulse"></div>
          </div>
          <div class="text-[8px] text-slate-400 font-mono tracking-widest">ENCRYPTED_STREAM_X4</div>
        </div>

        <div class="absolute bottom-0 left-0 w-full p-8 pointer-events-none">
          <div class="bg-white border border-slate-200 p-6 shadow-xl relative">
            <div
              class="absolute -top-3 left-4 bg-black text-white px-2 py-0.5 text-[8px] font-bold tracking-widest uppercase"
            >
              Target_Data_Node
            </div>
            <div class="flex justify-between items-end">
              <div>
                <h3 class="text-3xl font-black tracking-tighter text-slate-900 italic">
                  B17X_SYNT_B3
                </h3>
                <p class="text-[9px] text-slate-500 font-bold uppercase tracking-widest mt-1">
                  Vector-Cell Delivery Unit-CT
                </p>
              </div>
              <div class="text-[9px] text-slate-400 font-mono text-right">
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
          class="absolute inset-0 z-50 flex items-center justify-center bg-slate-900/10 backdrop-blur-sm p-6"
        >
          <div
            class="bg-white border-2 border-slate-900 w-full max-w-md shadow-[16px_16px_0px_0px_rgba(15,23,42,0.1)] relative"
          >
            <div class="bg-slate-900 p-4 text-white flex justify-between items-center">
              <span class="text-[9px] font-black tracking-[0.4em] uppercase"
                >Security_Protocol_Success</span
              >
              <div class="text-[9px] bg-white/20 px-2 py-1">RED_X4</div>
            </div>

            <div class="p-10 text-center">
              <div class="text-5xl mb-6 font-black">✓</div>
              <h2 class="text-2xl font-black tracking-tighter text-slate-900 mb-2">
                ACCESS_GRANTED
              </h2>
              <p class="text-[10px] text-slate-500 uppercase tracking-widest mb-8">
                Establishing link to identity terminal...
              </p>

              <div
                class="w-full border border-slate-900 py-4 text-[10px] font-black uppercase tracking-[0.3em] bg-slate-50"
              >
                Redirecting ({{ countdown }}s)
              </div>
            </div>

            <div class="bg-slate-50 p-3 flex justify-center gap-1">
              <div
                v-for="i in 5"
                :key="i"
                class="w-1.5 h-1.5 border border-slate-300"
                :class="{ 'bg-slate-900 animate-pulse': i <= 4 - countdown }"
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
