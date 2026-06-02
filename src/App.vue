<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
import { listen } from "@tauri-apps/api/event";

const sec = 1200;

const time = ref(sec);
const running = ref(false);
const isPaused = ref(false);
const timer = ref(null);

let unlistenTick;
let unlistenFinished;

onMounted(async () => {
  unlistenTick = await listen("timer-tick", (event) => {
    time.value = event.payload;
    running.value = true;
  });

  unlistenFinished = await listen("timer-finished", () => {
    running.value = false;
  });
});

onUnmounted(() => {
  if (unlistenTick) unlistenTick();
  if (unlistenFinished) unlistenFinished();
});

async function toggleTimer() {
  if (running.value && !isPaused.value) {
    await invoke("toggle_pause");
    isPaused.value = true;
    return;
  }

  if (running.value && isPaused.value) {
    await invoke("toggle_pause");
    isPaused.value = false;
    return;
  }

  await invoke("start_timer", {
    initialSeconds: sec,
    task: "Working on rwn",
    urlToBlock: "x.com",
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
  <main
    class="bg-linear-to-r from-[#af4949] to-[#F88379] text-white h-screen w-screen text-sm tracking-wide flex flex-col items-center justify-center"
  >
    <div class="mx-auto w-[75%] flex items-center gap-7">
      <button class="font-bold">Tasks</button>
      <button class="text-gray-200">Analytics</button>
    </div>

    <!-- <div class="flex items-center gap-3 mt-5"> -->
    <!-- <p class="font-bold text-sm">tasks done 3/5</p> -->
    <!-- · -->
    <!-- <p class="font-bold text-sm">28.50 mins focused</p> -->
    <!-- · -->
    <!-- <p class="font-bold text-sm">Add new ⌘ n</p> -->
    <!-- </div> -->

    <div
      class="flex flex-col items-center justify-center bg-[#af4949] rounded w-[75%] mt-5"
    >
      <div class="w-full py-5 px-3 shadow-md flex items-center justify-between">
        <input type="radio" class="scale-150" />

        <play-icon />
        <input
          type="text"
          class="outline-none w-[90%]"
          autofocus
          placeholder="Add new task here... Protip: add daily at the end for recurring tasks"
        />
      </div>
      <div class="w-full py-5 px-3 shadow-md flex items-start justify-between">
        <input type="radio" class="scale-150 mt-1" />

        <play-icon />
        <p class="break-all w-[90%]">
          Lorem Ipsum is simply dummy text of the printing and typesetting
          industry.
        </p>
      </div>
      <div class="w-full py-5 px-3 shadow-md flex items-start justify-between">
        <input type="radio" class="scale-150 mt-1" />

        <play-icon />
        <p class="break-all w-[90%]">
          Lorem Ipsum is simply dummy text of the printing and typesetting
          industry. Lorem Ipsum has been the industry's standard dummy text ever
          since 1966, when designers at Letraset and James Mosley, the librarian
          at St Bride Printing Library, took a 1914 Cicero translation and
          scrambled it to make dummy text for Letraset's Body Type sheets. It
          has survived not only many decades, but also the leap into electronic
          typesetting, remaining essentially unchanged. It was popularised
          thanks to these sheets and more recently with desktop publishing
          software including versions of Lorem Ipsum.
        </p>
      </div>
      <div class="w-full py-5 px-3 shadow-md flex items-start gap-3 opacity-70">
        <input type="radio" checked class="scale-150 mt-1" />

        <play-icon />
        <p class="break-all w-[90%]">
          It is a long established fact that a reader will be distracted by the
          readable content of a page when looking at its layout.
        </p>
      </div>
    </div>
  </main>

  <main
    v-if="false"
    class="bg-[#af4949] text-white h-screen w-screen text-sm tracking-wide"
  >
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
