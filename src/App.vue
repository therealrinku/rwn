<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

// async function greet() {
//  await invoke("greet", { name: name.value });
// }
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
  let minutes = Math.floor(timer.value / 60);
  let seconds = timer.value % 60;
  if (minutes.length === 1) minutes = `0${minutes}`;
  if (seconds === 0) seconds = seconds + "0";
  return `${minutes}:${seconds}`.replace("0:", "");
});

const progress = computed(() => {
  const elapsed = sec - timer.value;
  return Math.floor((elapsed / sec) * 100);
});
</script>

<template>
  <main class="-[#848884] h-screen w-screen text-sm tracking-wide">
    <div
      v-if="!running"
      data-tauri-drag-region
      class="fixed top-0 left-0 w-full h-10 p-2 flex items-center gap-3"
    >
      <button
        class="bg-red-500 rounded-full cursor-pointer h-5 w-5 flex flex-col items-center justify-center"
      >
        <svg
          width="16px"
          height="16px"
          viewBox="0 0 1024 1024"
          class="icon"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          fill="#fff"
        >
          <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
          <g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g>
          <g id="SVGRepo_iconCarrier">
            <path
              d="M720.7 354.8L669 303.1l-157 157-157-157-51.7 51.7 157 157-157 157 51.7 51.7 157-157 157 157 51.7-51.7-157-157z"
              fill="#fff"
            ></path>
          </g>
        </svg>
      </button>
      <button
        class="bg-gray-500 rounded-full cursor-pointer h-5 w-5 flex flex-col items-center justify-center"
      >
        <svg
          width="16px"
          height="16px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
          <g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g>
          <g id="SVGRepo_iconCarrier">
            <path
              d="M6 12L18 12"
              stroke="#fff"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
          </g>
        </svg>
      </button>
      <button
        class="bg-gray-500 rounded-full cursor-pointer h-5 w-5 flex flex-col items-center justify-center"
      >
        <svg
          width="16px"
          height="16px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
          <g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g>
          <g id="SVGRepo_iconCarrier">
            <g id="Arrow / Expand">
              <path
                id="Vector"
                d="M10 19H5V14M14 5H19V10"
                stroke="#fff"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              ></path>
            </g>
          </g>
        </svg>
      </button>
    </div>

    <div
      v-if="running"
      class="flex flex-col items-center justify-center gap-5 w-full h-full"
    >
      <p class="text-4xl font-bold tracking-widest">{{ formattedTime }}</p>
      <div class="w-32 h-1 bg-gray-200">
        <div
          :style="{ width: progress + '%' }"
          class="w-10 h-full bg-green-300"
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
          height="40px"
          width="40px"
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
        <span> 25.00 </span>
      </button>
    </div>
  </main>
</template>
