<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import PlayIcon from "./components/icons/play-icon.vue";
import PauseIcon from "./components/icons/pause-icon.vue";
import LeftIcon from "./components/icons/chevron-left-icon.vue";
import RightIcon from "./components/icons/chevron-right-icon.vue";

const sec = 1200;

export default defineComponent({
  name: "TimerTodoApp",
  components: {
    PlayIcon,
    PauseIcon,
    LeftIcon,
    RightIcon,
  },
  data() {
    return {
      formattedTime: null,
      running: false,
      isPaused: false,
      timer: null as any,
      todos: [] as any[],
      todoTitle: "",
      activeTimerTask: null as any,
      currentDate: new Date(),
      unlistenTick: null as any,
      unlistenFinished: null as any,
    };
  },
  computed: {
    todayTodos() {
      return this.todos.filter((todo) => {
        const todoDate = new Date(todo.date).setHours(0, 0, 0, 0);
        const tday = new Date(this.currentDate).setHours(0, 0, 0, 0);
        return todoDate === tday;
      });
    },
    showPauseIcon(): boolean {
      return this.running && !this.isPaused;
    },
    formattedDate(): string {
      return this.currentDate.toLocaleDateString("en-GB", {
        weekday: "short",
        day: "numeric",
        month: "short",
      });
    },
  },
  async mounted() {
    document.addEventListener("keydown", this.handleKeyboardShortcuts);

    this.unlistenTick = await listen("timer-tick", (event: any) => {
      this.running = true;
      this.formattedTime = event.payload;

      this.activeTimerTask.timeSpent += 1;
      this.syncTodosAndSave();
    });

    this.unlistenFinished = await listen("timer-finished", () => {
      this.running = false;
      this.activeTimerTask.finishTimestamp = null;
      this.syncTodosAndSave();
    });

    this.fetchTodos();
  },
  unmounted() {
    if (this.unlistenTick) this.unlistenTick();
    if (this.unlistenFinished) this.unlistenFinished();
    document.removeEventListener("keydown", this.handleKeyboardShortcuts);
  },
  methods: {
    getFormattedTime(): string {
      if (!this.activeTimerTask) return "00.00";

      const remainingSeconds = Math.floor(
        this.activeTimerTask.finishTimestamp / 1000 - Date.now() / 1000,
      );

      const mins = Math.floor(remainingSeconds / 60);
      const secs = remainingSeconds % 60;

      const formattedMins = String(mins).padStart(2, "0");
      const formattedSecs = String(secs).padStart(2, "0");
      return `${formattedMins}.${formattedSecs}`;
    },
    fetchTodos() {
      const saved = localStorage.getItem("todos");
      this.todos = saved ? JSON.parse(saved) : [];
    },
    syncTodosAndSave() {
      const todoIndex = this.todos.findIndex(
        (todo) => todo.id === this.activeTimerTask.id,
      );
      const updatedTodos = [...this.todos];
      updatedTodos[todoIndex] = {
        ...updatedTodos[todoIndex],
        timeSpent: this.activeTimerTask.timeSpent,
      };
      this.todos = updatedTodos;
      localStorage.setItem("todos", JSON.stringify(this.todos));
    },
    startTimerOnTask(todo: any) {
      this.activeTimerTask = todo;
      this.toggleTimer();
    },
    deleteTodo(id: number) {
      const filtered = this.todos.filter((tdo) => tdo.id !== id);
      this.todos = filtered;
      localStorage.setItem("todos", JSON.stringify(this.todos));
    },
    checkUncheck(id: number) {
      const todoIndex = this.todos.findIndex((tdo) => tdo.id === id);
      const updated = [...this.todos];
      updated[todoIndex].done = !Boolean(updated[todoIndex].done);
      this.todos = updated;
      localStorage.setItem("todos", JSON.stringify(this.todos));
    },
    addTodo() {
      if (!this.todoTitle.trim()) {
        return;
      }
      const newTodo = {
        id: Math.floor(
          new Date().getTime() + Math.random() * 200 + Math.random() * 100,
        ),
        title: this.todoTitle,
        timeSpent: 0,
        finishTimestamp: null,
        date: this.currentDate,
        done: false,
      };
      this.todos = [...this.todos, newTodo];
      localStorage.setItem("todos", JSON.stringify(this.todos));
      this.todoTitle = "";
    },
    async stopTimer() {
      if (this.running) {
        await invoke("toggle_pause");
        this.running = false;
        this.isPaused = true;
        return;
      }

      await invoke("stop_timer");
      const todoIndex = this.todos.findIndex(
        (todo) => todo.id === this.activeTimerTask.id,
      );
      const updatedTodos = [...this.todos];
      updatedTodos[todoIndex] = {
        ...updatedTodos[todoIndex],
        finishTimestamp: null,
      };
      this.todos = updatedTodos;
      localStorage.setItem("todos", JSON.stringify(this.todos));

      this.activeTimerTask = null;
      this.running = false;
      this.isPaused = false;
    },
    async toggleTimer() {
      if (this.running) {
        const isPaused = await invoke("toggle_pause");
        this.isPaused = isPaused;
        return;
      }

      if (!this.activeTimerTask.finishTimestamp) {
        this.activeTimerTask.finishTimestamp = Date.now() + sec * 1000;
      }

      await invoke("start_timer", {
        finishTimestamp: new Date(
          this.activeTimerTask.finishTimestamp,
        ).getTime(),
        taskTitle: this.activeTimerTask.title,
      });

      this.formattedTime = this.getFormattedTime();
      this.running = true;
      this.isPaused = false;
    },
    previousDay() {
      const date = new Date(this.currentDate);
      date.setDate(date.getDate() - 1);
      this.currentDate = date;
    },
    nextDay() {
      const date = new Date(this.currentDate);
      date.setDate(date.getDate() + 1);
      this.currentDate = date;
    },
    handleKeyboardShortcuts(event: KeyboardEvent) {
      if (event.key === "ArrowLeft") {
        this.previousDay();
      } else if (event.key === "ArrowRight") {
        this.nextDay();
      } else if (event.key === "r") {
        this.stopTimer();
      }
    },
  },
});
</script>

<template>
  <main
    v-if="!activeTimerTask"
    class="bg-linear-to-r from-[#af4949] to-[#F88379] text-white min-h-screen min-w-screen text-sm tracking-wide flex flex-col items-center py-20"
  >
    <div class="flex items-center w-[75%]">
      <button @click="previousDay">
        <LeftIcon />
      </button>

      <b class="w-24 flex items-center justify-center">{{ formattedDate }}</b>

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
          <PlayIcon />
        </button>

        <input
          v-model="todoTitle"
          type="text"
          class="outline-none w-[90%]"
          autofocus
          :placeholder="
            todayTodos.length === 5
              ? 'You have added 5 tasks for today.'
              : 'Add new task . . .'
          "
          :disabled="todayTodos.length === 5"
        />
      </form>

      <div
        v-for="todo in todayTodos"
        :key="todo.id"
        class="w-full py-5 px-3 shadow flex items-start justify-between relative"
        @dblclick="deleteTodo(todo.id)"
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
          <PlayIcon />
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
      <span class="bg-[#af4949] px-3 py-1 rounded">
        {{ activeTimerTask.title }}
      </span>

      <span class="text-5xl font-mono font-bold">{{ formattedTime }}</span>

      <div class="flex items-center gap-3">
        <button class="cursor-pointer" @click="toggleTimer">
          <Transition v-if="showPauseIcon">
            <PauseIcon />
          </Transition>

          <Transition v-else>
            <PlayIcon width="42px" height="42px" />
          </Transition>
        </button>
      </div>
    </div>
  </main>
</template>
