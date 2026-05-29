<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

//  await invoke("greet", { name: name.value });

const sec = 70;

const timer = ref(sec);
const running = ref(false);

async function startTimer() {
  running.value = true;
  setInterval(() => {
    if (timer.value > 0) {
      timer.value--;
    }
  }, 1000);
}

const formattedTime = computed(() => {
  const mins = Math.floor(timer.value / 60);
  const secs = timer.value % 60;

  const formattedMins = String(mins).padStart(2, "0");
  const formattedSecs = String(secs).padStart(2, "0");
  return `${formattedMins}.${formattedSecs}`;
});

const progress = computed(() => {
  const elapsed = sec - timer.value;
  return Math.floor((elapsed / sec) * 100);
});
</script>

<template>
  <main class="h-screen w-screen text-sm tracking-wide">
    <div
      v-if="running"
      class="flex flex-col items-center justify-center gap-5 w-full h-full"
    >
      <p class="text-4xl font-bold tracking-widest font-mono">
        {{ formattedTime }}
      </p>
      <div class="fixed bottom-0 left-0 w-full h-1 bg-gray-300">
        <div
          :style="{ width: progress + '%' }"
          class="w-10 h-full bg-green-400"
        ></div>
      </div>
    </div>

    <div
      v-else
      class="flex flex-col items-center justify-center gap-3 w-full h-full"
    >
      <button
        class="cursor-pointer flex items-center gap-3 text-xl font-bold"
        @click="startTimer"
      >
        <svg
          height="38px"
          width="38px"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 512 512"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          enable-background="new 0 0 512 512"
          fill="#000000"
        >
          <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
          <g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g>
          <g id="SVGRepo_iconCarrier">
            <g>
              <g fill="#231F20">
                <path
                  d="m354.2,247.4l-135.1-92.4c-4.2-3.1-15.4-3.1-16.3,8.6v184.8c1,11.7 12.4,11.9 16.3,8.6l135.1-92.4c3.5-2.1 8.3-10.7 0-17.2zm-130.5,81.3v-145.4l106.1,72.7-106.1,72.7z"
                ></path>
                <path
                  d="M256,11C120.9,11,11,120.9,11,256s109.9,245,245,245s245-109.9,245-245S391.1,11,256,11z M256,480.1 C132.4,480.1,31.9,379.6,31.9,256S132.4,31.9,256,31.9S480.1,132.4,480.1,256S379.6,480.1,256,480.1z"
                ></path>
              </g>
            </g>
          </g>
        </svg>
        <span class="text-4xl font-mono">25.00 </span>
      </button>
    </div>
  </main>
</template>
