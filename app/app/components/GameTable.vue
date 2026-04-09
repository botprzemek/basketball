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
  games: Game[];
}>();
</script>

<template>
  <div class="w-full max-w-5xl mx-auto mt-12 relative">
    <div class="flex items-center gap-3 mb-4">
      <div class="h-4 w-1 bg-slate-900"></div>
      <span class="text-[10px] font-black tracking-[0.3em] text-slate-900 uppercase">
        Live_Feed // Database_Query_Result
      </span>
    </div>

    <div
      class="border-2 border-slate-900 shadow-[12px_12px_0px_0px_rgba(15,23,42,0.05)] overflow-hidden bg-white"
    >
      <div class="grid grid-cols-7 bg-slate-900 text-white p-4">
        <div
          class="col-span-2 text-right text-[9px] font-black uppercase tracking-[0.2em] opacity-60"
        >
          Host_Unit
        </div>
        <div
          class="col-span-3 text-center text-[9px] font-black uppercase tracking-[0.2em] opacity-60"
        >
          Data_Point
        </div>
        <div
          class="col-span-2 text-left text-[9px] font-black uppercase tracking-[0.2em] opacity-60"
        >
          Guest_Unit
        </div>
      </div>

      <div v-if="games.length > 0" class="divide-y divide-slate-100">
        <GameRow v-for="game in games" :key="game.id" :game="game" />
      </div>

      <div v-else class="p-20 text-center bg-slate-50">
        <div class="text-slate-300 font-black text-4xl italic tracking-tighter mb-2">
          NO_DATA_FOUND
        </div>
        <p class="text-[10px] text-slate-400 uppercase tracking-widest">
          System is idling. Awaiting stream injection...
        </p>
      </div>

      <div
        class="bg-slate-50 p-3 border-t-2 border-slate-900 flex justify-between items-center px-6"
      >
        <div class="text-[8px] font-mono text-slate-400 uppercase">Encrypted_Table_v2.1</div>
        <div class="flex gap-2">
          <div v-for="i in 3" :key="i" class="w-1 h-1 bg-slate-900"></div>
        </div>
      </div>
    </div>
  </div>
</template>
