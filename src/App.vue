<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import Header from "./components/Common/Header.vue";
import MonthView from "./components/Calendar/MonthView.vue";
import TodoPanel from "./components/Todo/TodoPanel.vue";
import { useCalendarStore } from "./stores/calendar";
import { useTodoStore } from "./stores/todo";

const calendarStore = useCalendarStore();
const todoStore = useTodoStore();
const currentDate = ref(new Date());

const todoPanelWidth = ref(parseInt(localStorage.getItem("todoPanelWidth") || "300", 10));
const isResizing = ref(false);

function startResize(e: MouseEvent) {
  isResizing.value = true;
  document.body.style.cursor = "col-resize";
  document.body.style.userSelect = "none";

  const startX = e.clientX;
  const startWidth = todoPanelWidth.value;

  function onMouseMove(ev: MouseEvent) {
    const delta = startX - ev.clientX;
    const newWidth = Math.max(220, Math.min(500, startWidth + delta));
    todoPanelWidth.value = newWidth;
  }

  function onMouseUp() {
    isResizing.value = false;
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
    localStorage.setItem("todoPanelWidth", String(todoPanelWidth.value));
    document.removeEventListener("mousemove", onMouseMove);
    document.removeEventListener("mouseup", onMouseUp);
  }

  document.addEventListener("mousemove", onMouseMove);
  document.addEventListener("mouseup", onMouseUp);
}

onMounted(async () => {
  await Promise.all([
    calendarStore.loadMonth(currentDate.value),
    todoStore.loadTodos(),
  ]);
});

watch(currentDate, (d) => {
  calendarStore.loadMonth(d);
});
</script>

<template>
  <div class="app-container">
    <Header :current-date="currentDate" @update:date="currentDate = $event" />
    <div class="main-content">
      <MonthView :current-date="currentDate" />
      <div
        class="resizer"
        :class="{ active: isResizing }"
        @mousedown="startResize"
      />
      <TodoPanel :style="{ width: todoPanelWidth + 'px' }" />
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    "Helvetica Neue", Arial, "PingFang SC", "Hiragino Sans GB", sans-serif;
  background-color: #f5f5f7;
  color: #1d1d1f;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.resizer {
  width: 5px;
  cursor: col-resize;
  background-color: transparent;
  position: relative;
  z-index: 10;
  flex-shrink: 0;
  transition: background-color 0.15s ease;
}

.resizer:hover,
.resizer.active {
  background-color: #007aff;
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.18);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.28);
}

::-webkit-scrollbar-track {
  background-color: transparent;
}
</style>
