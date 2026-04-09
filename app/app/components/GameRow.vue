<script setup lang="ts">
interface Game {
  id: string;
  home_name: string;
  away_name: string;
  score: string;
  status: string;
  created_at: number;
}

defineProps<{
  game: Game;
}>();
</script>

<template>
  <div
    class="grid grid-cols-7 items-center bg-white border-b border-slate-200 hover:bg-slate-50 transition-all p-5 group relative overflow-hidden"
  >
    <div
      class="absolute left-0 top-0 bottom-0 w-1 bg-slate-900 -translate-x-full group-hover:translate-x-0 transition-transform"
    ></div>

    <div
      class="col-span-2 text-right font-black text-slate-900 uppercase tracking-tighter text-sm lg:text-base"
    >
      {{ game.home_name }}
    </div>

    <div class="col-span-3 flex flex-col items-center justify-center px-4">
      <div
        class="font-mono tracking-[0.2em] px-6 py-2 border-2 text-lg lg:text-xl transition-colors"
        :class="
          game.status === 'FINISHED'
            ? 'border-slate-900 bg-slate-900 text-white shadow-[4px_4px_0px_0px_rgba(0,0,0,0.1)]'
            : 'border-slate-200 bg-slate-50 text-slate-400'
        "
      >
        {{ game.score }}
      </div>

      <div class="flex items-center gap-2 mt-2">
        <div v-if="game.status === 'LIVE'" class="w-1.5 h-1.5 bg-red-500 animate-pulse"></div>
        <span class="text-[9px] uppercase font-black tracking-[0.2em] text-slate-400">
          {{ game.status }} // REF_{{ game.id.slice(0, 4) }}
        </span>
      </div>
    </div>

    <div
      class="col-span-2 text-left font-black text-slate-900 uppercase tracking-tighter text-sm lg:text-base"
    >
      {{ game.away_name }}
    </div>
  </div>
</template>
