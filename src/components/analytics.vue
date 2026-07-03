<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Analytics28Days",
  props: {
    todos: {
      type: Array,
      required: true,
    },
  },
  data() {
    return {
      days: [] as any[],
    };
  },

  mounted() {
    this.buildLast28Days();
  },
  watch: {
    todos() {
      this.buildLast28Days();
    },
  },
  methods: {
    buildLast28Days() {
      const result: any[] = [];

      for (let i = 27; i >= 0; i--) {
        const date = new Date();
        date.setDate(date.getDate() - i);

        const normalized = new Date(date);
        normalized.setHours(0, 0, 0, 0);

        const dayTodos = this.todos.filter((t) => {
          const todoDate = new Date(t.date);
          todoDate.setHours(0, 0, 0, 0);
          return todoDate.getTime() === normalized.getTime();
        });

        const completed = dayTodos.filter((t) => t.done).length;
        const total = dayTodos.length;

        const completionRate =
          total === 0 ? 0 : Math.round((completed / total) * 100);

        const totalSeconds = dayTodos.reduce(
          (sum, t) => sum + (t.timeSpent),
          0,
        );

        result.push({
          date: normalized,
          label: normalized.toLocaleDateString("en-GB", {
            day: "2-digit",
            month: "short",
          }),
          completed,
          total,
          completionRate,
          hours: +(totalSeconds / 3600).toFixed(2),
        });
      }

      this.days = result.reverse();
    },
  },
});
</script>

<template>
  <div class="w-full p-6 text-white bg-linear-to-r from-[#af4949] to-[#F88379]">
    <h2 class="text-xl font-bold mb-4">Last 28 Days Analytics</h2>

    <div class="grid gap-3">
      <div
        v-for="day in days"
        :key="day.label"
        class="bg-[#af4949] p-3 rounded flex flex-col gap-1 shadow"
      >
        <div class="flex justify-between">
          <span class="font-semibold">{{ day.label }}</span>
          <span>{{ day.completionRate }}%</span>
        </div>

        <div class="text-xs opacity-80">
          {{ day.completed }} / {{ day.total }} tasks completed
        </div>

        <div class="w-full bg-white/20 h-1 rounded overflow-hidden">
          <div
            class="h-full bg-gray-300"
            :style="{ width: day.completionRate + '%' }"
          />
        </div>

        <div class="text-xs mt-1">⏱ {{ day.hours }} hours spent</div>
      </div>
    </div>
  </div>
</template>
