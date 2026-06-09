<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
import StopIcon from "./components/icons/stop-icon.vue";
import LeftIcon from "./components/icons/chevron-left-icon.vue";
import RightIcon from "./components/icons/chevron-right-icon.vue";
import { listen } from "@tauri-apps/api/event";

const sec = 1200;

const time = ref(sec);
const running = ref(false);
const isPaused = ref(false);
const timer = ref(null);
const todos = ref([]);
const todoTitle = ref("");
const activeTimerTask = ref(null);

let unlistenTick;
let unlistenFinished;

onMounted(async () => {
  document.addEventListener('keydown', handleKeyboardShortcuts);
  
  unlistenTick = await listen("timer-tick", (event) => {
    time.value = event.payload;
    running.value = true;

    // stop when time goes to 0
    if (event.payload === 0) {
      //stop
      return;
    }

    // update the time spent, remaining
    activeTimerTask.value.worked_for_sec += 1;
    activeTimerTask.value.remaining_sec -= 1;

    // update the todos, local storage
    const todoIndex = todos.value.findIndex(
      (todo) => todo.id === activeTimerTask.value.id,
    );
    const updatedTodos = [...todos.value];
    updatedTodos[todoIndex] = {
      ...updatedTodos[todoIndex],
      worked_for_sec: activeTimerTask.value.worked_for_sec,
      remaining_sec: activeTimerTask.value.remaining_sec,
    };
    todos.value = updatedTodos;
    localStorage.setItem("todos", JSON.stringify(todos.value));
  });

  unlistenFinished = await listen("timer-finished", () => {
    running.value = false;
  });

  const saved = localStorage.getItem("todos");
  todos.value = saved ? JSON.parse(saved) : [];
});

onUnmounted(() => {
  if (unlistenTick) unlistenTick();
  if (unlistenFinished) unlistenFinished();
  document.removeEventListener('keydown', handleKeyboardShortcuts);
});

function startTimerOnTask(todo) {
  activeTimerTask.value = todo;
  toggleTimer();
}

function deleteTodo(id) {
  const filtered = todos.value.filter((tdo) => tdo.id !== id);
  todos.value = filtered;
  localStorage.setItem("todos", JSON.stringify(todos.value));
}

function checkUncheck(id) {
  const todoIndex = todos.value.findIndex((tdo) => tdo.id === id);
  const updated = [...todos.value];
  updated[todoIndex].done = !Boolean(updated[todoIndex].done);
  todos.value = updated;
  localStorage.setItem("todos", JSON.stringify(todos.value));
}

function addTodo() {
  if (!todoTitle.value.trim()) {
    return;
  }
  const newTodo = {
    id: Math.floor(
      new Date().getTime() + Math.random() * 200 + Math.random() * 100,
    ),
    title: todoTitle.value,
    worked_for_sec: 0,
    remaining_sec: sec,
    date: new Date(),
    done: false,
  };
  todos.value = [...todos.value, newTodo];
  localStorage.setItem("todos", JSON.stringify(todos.value));
  todoTitle.value = "";
}

async function stopTimer() {
    await invoke("stop_timer");
    // update remaining_sec in localstorage // TODO: CONSOLIDATE
    const todoIndex = todos.value.findIndex(
      (todo) => todo.id === activeTimerTask.value.id,
    );
    const updatedTodos = [...todos.value];
    updatedTodos[todoIndex] = {
      ...updatedTodos[todoIndex],
      //reset
      remaining_sec: sec,
    };
    todos.value = updatedTodos;
    localStorage.setItem("todos", JSON.stringify(todos.value));
  
    activeTimerTask.value = null;
}

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

  // fallback to 'sec' if remaining_sec goes to 0 aka run new session
  // maybe make it === 0 ???
  if(activeTimerTask.value.remaining_sec <= 0) {
    activeTimerTask.value.remaining_sec = sec;
  }

  await invoke("start_timer", {
    initialSeconds: activeTimerTask.value.remaining_sec,
    task: activeTimerTask.value.title,
  });

  running.value = true;
  isPaused.value = false;
}

const formattedTime = computed(() => {
  const mins = Math.floor(activeTimerTask.value.remaining_sec / 60);
  const secs = activeTimerTask.value.remaining_sec % 60;

  const formattedMins = String(mins).padStart(2, "0");
  const formattedSecs = String(secs).padStart(2, "0");
  return `${formattedMins}.${formattedSecs}`;
});

const progress = computed(() => {
  const elapsed = sec - time.value;
  return Math.floor((elapsed / sec) * 100);
});

const showPauseIcon = computed(() => running.value && !isPaused.value);

const currentDate = ref(new Date());

const formattedDate = computed(() => {
  return currentDate.value.toLocaleDateString("en-GB", {
    weekday: "short",
    day: "numeric",
    month: "long",
  });
});

function previousDay() {
  const date = new Date(currentDate.value);
  date.setDate(date.getDate() - 1);
  currentDate.value = date;
}

function nextDay() {
  const date = new Date(currentDate.value);
  date.setDate(date.getDate() + 1);
  currentDate.value = date;
}

function handleKeyboardShortcuts(event) {
  if (event.key === 'ArrowLeft') {
    previousDay();
  } else if (event.key === 'ArrowRight') {
    nextDay();
  }
  // more to come
}

</script>

<template>
  <main
    v-if="!activeTimerTask"
    class="bg-linear-to-r from-[#af4949] to-[#F88379] text-white h-screen w-screen text-sm tracking-wide flex flex-col items-center pt-20"
  >

  <div class="flex items-center w-[75%] ">
  <button @click="previousDay">
    <LeftIcon />
  </button>

  <b>{{ formattedDate }}</b>

  <button @click="nextDay">
    <RightIcon />
  </button>
</div>

    <div
      class="flex flex-col items-center justify-center bg-[#af4949] rounded w-[75%] mt-5"
    >
      <form
        class="w-full py-5 px-3 shadow flex items-start justify-between"
        @submit.prevent="addTodo"
      >
        <input type="checkbox" disabled class="scale-150 mt-1" />

        <button class="opacity-50">
          <play-icon />
        </button>

        <input
          v-model="todoTitle"
          type="text"
          class="outline-none w-[90%]"
          autofocus
          :placeholder="todos.length === 5 ? 'You have added 5 tasks for today.': 'Add new task . . .'"
          :disabled="todos.length === 5"
        />
      </form>
      <div
        v-for="todo in todos"
        :key="todo.id"
        class="w-full py-5 px-3 shadow flex items-start justify-between relative"
        v-on:dblclick="deleteTodo(todo.id)"
      >
        <input
          type="checkbox"
          class="scale-150 mt-1"
          :checked="todo.done"
          @change="checkUncheck(todo.id)"
        />

        <button
          :class="{ 'opacity-50': todo.done }"
          :disabled="todo.done"
          @click="startTimerOnTask(todo)"
        >
          <play-icon />
        </button>
        <p
          class="break-all w-[90%]"
          :class="{ 'opacity-50 line-through': todo.done }"
        >
          {{ todo.title }}
        </p>
      </div>
    </div>
  </main>

  <main
    v-if="activeTimerTask"
    class="bg-linear-to-r from-[#af4949] to-[#F88379] text-white h-screen w-screen text-sm tracking-wide flex flex-col items-center justify-center"
  >
    <div class="flex flex-col items-center justify-center w-full h-full gap-5">
      <span class="bg-[#af4949] px-3 py-1 rounded">{{
        activeTimerTask.title
      }}</span>

      <span class="text-5xl font-mono font-bold">{{ formattedTime }}</span>

      <div class="flex items-center gap-3">
      <button class="cursor-pointer" @click="toggleTimer">
        <Transition v-if="showPauseIcon">
          <pause-icon />
        </Transition>

        <Transition v-else>
          <play-icon width="42px" height="42px" />
        </Transition>
      </button>

      <button class="cursor-pointer" @click="stopTimer">
        <stop-icon />
      </button>
      </div>
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
