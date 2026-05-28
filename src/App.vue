<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

// async function greet() {
//  await invoke("greet", { name: name.value });
// }

const timer = ref(300);
const running = ref(false);

async function startTimer() {
console.log("dd:w")
  running.value = true;
  setInterval(() => {
     if(timer.value>0) {
       timer.value--;    
     }
  }, 1000)
}

const formattedTime = computed(() => { 
const minutes =  Math.floor(timer.value / 60);
const seconds =  timer.value % 60;
return `${minutes}:${seconds}`
})

</script>

<template>
  <main class="bg-linear-to-l from-[#20002c] to-[#cbb4d4] h-screen w-screen text-sm tracking-wide">
    <div v-if="running" class="flex items-center justify-center gap-3 w-full h-full">
       <p class="text-4xl font-bold tracking-widest">{{ formattedTime }} </p>
    </div>
  
    <div v-else class="flex flex-col items-center justify-center gap-3 w-full h-full">
    <svg width="50px" height="50px" viewBox="0 0 36 36" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" class="iconify iconify--twemoji" preserveAspectRatio="xMidYMid meet" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"><path fill="#AAB8C2" d="M18 0C8.611 0 1 7.611 1 17h.029v17c0 1.1.9 2 2 2H33c1.1 0 2-.9 2-2V17c0-9.389-7.611-17-17-17z"></path><circle fill="#FFF" cx="18" cy="17" r="14"></circle><path fill="#66757F" d="M18 4c7.18 0 13 5.82 13 13s-5.82 13-13 13S5 24.18 5 17S10.82 4 18 4m0-2C9.729 2 3 8.729 3 17s6.729 15 15 15s15-6.729 15-15S26.271 2 18 2z"></path><path fill="#292F33" d="M19 6a1 1 0 0 0-2 0v1a1 1 0 0 0 2 0V6zm0 21a1 1 0 0 0-2 0v1a1 1 0 0 0 2 0v-1zM8 16H7a1 1 0 0 0 0 2h1a1 1 0 0 0 0-2zm21 0h-1a1 1 0 0 0 0 2h1a1 1 0 0 0 0-2zm-2.293 8.293L26 23.586A.999.999 0 1 0 24.586 25l.707.707a.999.999 0 1 0 1.414-1.414zM10 23.586l-.707.707a.999.999 0 1 0 1.414 1.414l.707-.707A.999.999 0 1 0 10 23.586zM9.293 9.707l.707.707A.999.999 0 1 0 11.414 9l-.707-.707a.999.999 0 1 0-1.414 1.414zM26 10.414l.707-.707a.999.999 0 1 0-1.414-1.414L24.586 9A.999.999 0 1 0 26 10.414z"></path><path fill="#DD2E44" d="M8.915 13.839a1 1 0 0 1 1.325-1.288l8.154 3.51a1 1 0 0 1-.791 1.837l-8.154-3.51a1 1 0 0 1-.534-.549z"></path></g></svg>

    <h1 class="font-bold italic">rwn</h1>

    <p>Minimalistic local only timer and blocker app.</p>

    <button class="-------px-5 py-1 tracking-wide cursor-pointer" @click="startTimer">Start timer</button>
   </div>
  </main>
</template>
