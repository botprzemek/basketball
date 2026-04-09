<script setup lang="ts">
const { data: games, pending, error } = useFetch("/api/games");
</script>

<template>
  <main class="min-h-screen bg-gray-50 py-12 px-4">
    <div class="max-w-4xl mx-auto mb-8 text-center">
      <h1 class="text-4xl font-black text-gray-900 uppercase tracking-tighter italic">
        Basketball League <span class="text-blue-600">Results</span>
      </h1>
      <p class="text-gray-500 mt-2">Live updates from NBA & FIBA match-ups</p>
    </div>

    <div v-if="pending" class="flex justify-center p-20">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
    </div>

    <div
      v-else-if="error || !games"
      class="bg-red-50 text-red-600 p-4 rounded-lg text-center border border-red-200"
    >
      Failed to load games. Is the API running?
    </div>

    <GameTable v-else :games="games" />
  </main>
</template>
