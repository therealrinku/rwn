<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
import { listen } from '@tauri-apps/api/event';

const sec = 1200;

const time = ref(sec);
const running = ref(false);
const isPaused = ref(false);
const timer = ref(null);

let unlistenTick;
let unlistenFinished;

onMounted(async () => {
  unlistenTick = await listen('timer-tick', (event) => {
    time.value = event.payload;
    running.value = true;
  });

  unlistenFinished = await listen('timer-finished', () => {
    running.value = false;
  });
});

onUnmounted(() => {
  if (unlistenTick) unlistenTick();
  if (unlistenFinished) unlistenFinished();
});

async function toggleTimer() {
  if (running.value && !isPaused.value) {
    await invoke('toggle_pause');
    isPaused.value = true;
    return;
  }

  if (running.value && isPaused.value) {
    await invoke('toggle_pause');
    isPaused.value = false;
    return;
  }

  await invoke('start_timer', { 
    initialSeconds: sec, 
    task: 'Working on rwn',
    urlToBlock: 'x.com'
  });

  running.value = true;
  isPaused.value = false;
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

const showPauseIcon = computed(() => running.value && !isPaused.value);
</script>

<template>
  <main class="bg-linear-to-r from-[#af4949] to-red-200 text-white h-screen w-screen text-sm tracking-wider flex flex-col items-center justify-center">

  <div class="flex items-center gap-3">
    <p class="font-bold text-lg">3/5</p> -
    <p class="font-bold text-lg">28.50 min</p>
  </div>

  <div class="w-full flex flex-col items-center justify-center">
     <div class="bg-[#af4949] mt-5 w-[50%] py-5 px-3 shadow rounded flex items-center justify-between">
        <p>Finish the work</p>
        <play-icon />
     </div>
     <div class="bg-[#af4949] mt-5 w-[50%] py-5 px-3 shadow rounded flex items-center justify-between">
        <p>Complete the assignment</p>
        <play-icon />
     </div>
     <div class="bg-[#af4949] mt-5 w-[50%] py-5 px-3 shadow rounded flex items-center justify-between">
        <p>Complete the assignment</p>
        <play-icon />
     </div>
     <div class="bg-[#af4949] mt-5 w-[50%] py-5 px-3 shadow rounded flex items-center justify-between">
        <p>Complete the assignment</p>
        <play-icon />
     </div>
     <div class="bg-[#af4949] mt-5 w-[50%] py-5 px-3 shadow rounded flex items-center justify-between">
        <p>Complete the assignment</p>
        <play-icon />
     </div>
   </div>
  </main>

  <main v-if="false" class="bg-[#af4949] text-white h-screen w-screen text-sm tracking-wide">
    <div
      class="flex flex-col items-center justify-center w-full h-full font-mono font-bold"
    >
      <span class="text-5xl">{{ formattedTime }}</span>

      <button class="cursor-pointer fixed mt-36" @click="toggleTimer">
        <Transition v-if="showPauseIcon">
          <pause-icon />
        </Transition>

        <Transition v-else>
          <play-icon />
        </Transition>
      </button>

    </div>
  </main>
</template>

<style>
.v-enter-active,
.v-leave-active {
  transition: opacity 1s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
