<script setup lang="ts">
const isTransitioning = ref(false);
const router = useRouter();

// Obsługa wyjścia (Out) - animacja przed zmianą strony
router.beforeEach((to, from, next) => {
  if (to.path === from.path) return next();

  isTransitioning.value = true;
  // Czekamy 600ms na pełne zasłonięcie ekranu kafelkami
  setTimeout(() => {
    next();
  }, 600);
});

// Obsługa wejścia (In) - odsłanianie po zmianie strony
router.afterEach(() => {
  // Krótki delay, aby strona zdążyła się wyrenderować pod spodem
  setTimeout(() => {
    isTransitioning.value = false;
  }, 100);
});
</script>

<template>
  <div class="app-root">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>

    <aside class="mosaic-overlay" :class="{ 'is-active': isTransitioning }" aria-hidden="true">
      <div v-for="i in 400" :key="i" class="mosaic-tile"></div>

      <div class="mosaic-status">
        <div class="flex items-center gap-4">
          <div class="w-1 h-4 bg-white animate-pulse"></div>
          <span class="tracking-[0.8em]">SWITCHING_NODE...</span>
        </div>
      </div>
    </aside>
  </div>
</template>

<style>
.app-root {
  position: relative;
}

.mosaic-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: grid;
  /* 20 kolumn x 20 wierszy = 400 małych kafelków */
  grid-template-columns: repeat(20, 1fr);
  grid-template-rows: repeat(20, 1fr);
  pointer-events: none;
  visibility: hidden;
}

/* Kiedy animacja trwa, overlay musi być widoczny */
.mosaic-overlay.is-active,
.mosaic-overlay:not(.is-active) {
  visibility: visible;
}

.mosaic-tile {
  background: #0f172a; /* Głęboki granat z Twojej palety */
  opacity: 0;
  transition: opacity 0.25s steps(3);
  transform: scale(1.05); /* Nakładanie kafelków, by nie było szpar */
}

/* --- ANIMACJA WYJŚCIA (Zamykanie krawędzie -> środek) --- */
.mosaic-overlay.is-active .mosaic-tile {
  opacity: 1;
  pointer-events: auto;
}

/* Logika opóźnień: im bliżej środka (kolumny 10,11), tym większy delay */
/* Generowane dynamicznie dla uproszczenia w CSS */
.mosaic-tile:nth-child(20n + 1),
.mosaic-tile:nth-child(20n + 20),
.mosaic-tile:nth-child(-n + 20),
.mosaic-tile:nth-child(n + 381) {
  transition-delay: 0s;
}

.mosaic-tile:nth-child(n + 100):nth-child(-n + 300) {
  transition-delay: 0.2s;
}

.mosaic-tile:nth-child(190),
.mosaic-tile:nth-child(191),
.mosaic-tile:nth-child(210),
.mosaic-tile:nth-child(211) {
  transition-delay: 0.4s;
}

/* --- ANIMACJA WEJŚCIA (Otwieranie środek -> krawędzie) --- */
.mosaic-overlay:not(.is-active) .mosaic-tile {
  opacity: 0;
  /* Kafelki znikają w odwrotnej kolejności */
  transition-delay: calc(var(--d, 0) * 1ms);
}

/* Przykładowe mapowanie opóźnień dla efektu wybuchu ze środka */
.mosaic-tile:nth-child(190),
.mosaic-tile:nth-child(191),
.mosaic-tile:nth-child(210),
.mosaic-tile:nth-child(211) {
  --d: 0;
}
.mosaic-tile:nth-child(1) {
  --d: 400;
}

.mosaic-status {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: white;
  font-size: 10px;
  font-weight: 900;
  z-index: 10000;
  opacity: 0;
  transition: opacity 0.2s;
}

.mosaic-overlay.is-active .mosaic-status {
  opacity: 1;
  transition-delay: 0.3s;
}

/* Globalne tło i reset */
body {
  background-color: #e8f1f2;
  margin: 0;
}
</style>
