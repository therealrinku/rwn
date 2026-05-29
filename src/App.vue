<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
//  await invoke("greet", { name: name.value });

const sec = 1200;

const time = ref(sec);
const running = ref(false);
const timer = ref(null);

function toggleTimer() {
  if (running.value) {
    clearInterval(timer.value);
    running.value = false;
    return;
  }

  running.value = true;
  timer.value = setInterval(() => {
    if (time.value > 0) {
      time.value--;
    } else {
      clearInterval(timer.value);
    }
  }, 1000);
}

const formattedTime = computed(() => {
  const mins = Math.floor(time.value / 60);
  const secs = time.value % 60;

  const formattedMins = String(mins).padStart(2, "0");
  const formattedSecs = String(secs).padStart(2, "0");
  return `${formattedMins}.${formattedSecs}`;
});

const progress = computed(() => {
  const elapsed = sec - time.value;
  return Math.floor((elapsed / sec) * 100);
});
</script>

<template>
  <main class="bg-[#af4949] text-white h-screen w-screen text-sm tracking-wide">
    <div
      class="flex flex-col items-center justify-center gap-5 w-full h-full font-mono font-bold"
    >
      <span class="text-5xl">{{ formattedTime }}</span>

      <button class="cursor-pointer" @click="toggleTimer">
        <play-icon v-if="!running" />
        <pause-icon v-else />
      </button>

      <div v-if="time !== sec" class="fixed bottom-12 w-48 h-1 bg-gray-300">
        <div
          :style="{ width: progress + '%' }"
          class="w-10 h-full bg-white"
        ></div>
      </div>
    </div>
  </main>
</template>
