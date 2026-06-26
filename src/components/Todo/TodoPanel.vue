<script setup lang="ts">
import { ref, computed } from "vue";
import { format, addDays, subDays } from "date-fns";
import { useTodoStore } from "@/stores/todo";
import { todayKey } from "@/utils/date";
import TodoItem from "./TodoItem.vue";

const todoStore = useTodoStore();

const newTodoTitle = ref("");
const newTodoType = ref<"global" | "daily">("daily");

const selectedDateObj = computed(() => new Date(todoStore.selectedDate + "T00:00:00"));

const displayDate = computed(() => format(selectedDateObj.value, "M 月 d 日"));
const weekDay = computed(() => {
  const days = ["周日", "周一", "周二", "周三", "周四", "周五", "周六"];
  return days[selectedDateObj.value.getDay()];
});
const isToday = computed(() => todoStore.selectedDate === todayKey());

async function addTodo() {
  if (!newTodoTitle.value.trim()) return;
  const result = await todoStore.createTodo(
    newTodoTitle.value.trim(),
    newTodoType.value,
    newTodoType.value === "daily" ? todoStore.selectedDate : undefined
  );
  if (result) {
    newTodoTitle.value = "";
  }
}

function goToPreviousDay() {
  const prev = subDays(selectedDateObj.value, 1);
  todoStore.setSelectedDate(format(prev, "yyyy-MM-dd"));
}

function goToNextDay() {
  const next = addDays(selectedDateObj.value, 1);
  todoStore.setSelectedDate(format(next, "yyyy-MM-dd"));
}

function goToToday() {
  todoStore.setSelectedDate(todayKey());
}
</script>

<template>
  <aside class="todo-panel">
    <div class="panel-header">
      <h2 class="panel-title">待办事项</h2>
    </div>

    <div class="date-selector">
      <button class="btn-nav" @click="goToPreviousDay" aria-label="前一天">
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
          <path d="M7 8L3 5L7 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
      <div class="date-display">
        <span class="date-text">{{ displayDate }}</span>
        <span class="week-day-text">{{ weekDay }}</span>
      </div>
      <button class="btn-nav" @click="goToNextDay" aria-label="后一天">
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
          <path d="M3 8L7 5L3 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
      <button v-if="!isToday" class="btn-today-small" @click="goToToday">今天</button>
    </div>

    <div class="add-todo">
      <div v-if="todoStore.error" class="error-bar">{{ todoStore.error }}</div>
      <div class="type-toggle">
        <button
          class="type-btn"
          :class="{ active: newTodoType === 'daily' }"
          @click="newTodoType = 'daily'"
        >当日</button>
        <button
          class="type-btn"
          :class="{ active: newTodoType === 'global' }"
          @click="newTodoType = 'global'"
        >全局</button>
      </div>
      <div class="input-row">
        <input
          v-model="newTodoTitle"
          class="todo-input"
          placeholder="添加待办..."
          @keydown.enter="addTodo"
        />
        <button class="btn-add" :disabled="!newTodoTitle.trim()" @click="addTodo" aria-label="添加">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M8 3V13M3 8H13" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
          </svg>
        </button>
      </div>
    </div>

    <div class="todo-sections">
      <div class="todo-section">
        <div class="section-header">
          <span class="section-title">当日待办</span>
          <span class="section-count">{{ todoStore.dailyTodos.length }}</span>
        </div>
        <div class="todo-list" v-if="todoStore.dailyTodos.length">
          <TodoItem v-for="todo in todoStore.dailyTodos" :key="todo.id" :todo="todo" />
        </div>
        <div v-else class="empty-state">暂无当日待办</div>
      </div>

      <div class="todo-section">
        <div class="section-header">
          <span class="section-title">全局待办</span>
          <span class="section-count">{{ todoStore.globalTodos.length }}</span>
        </div>
        <div class="todo-list" v-if="todoStore.globalTodos.length">
          <TodoItem v-for="todo in todoStore.globalTodos" :key="todo.id" :todo="todo" />
        </div>
        <div v-else class="empty-state">暂无全局待办</div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.todo-panel {
  width: 308px;
  display: flex;
  flex-direction: column;
  background-color: #fbfbfd;
  border-left: 1px solid #ececee;
}

.panel-header {
  padding: 18px 18px 0;
}

.panel-title {
  font-size: 15px;
  font-weight: 600;
  color: #1d1d1f;
}

.date-selector {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 18px;
}

.btn-nav {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 7px;
  background-color: #ffffff;
  color: #1d1d1f;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
}

.btn-nav:hover {
  background-color: #f0f0f2;
}

.date-display {
  display: flex;
  align-items: baseline;
  gap: 6px;
  flex: 1;
  justify-content: center;
}

.date-text {
  font-size: 14px;
  font-weight: 600;
  color: #1d1d1f;
}

.week-day-text {
  font-size: 12px;
  color: #86868b;
}

.btn-today-small {
  padding: 4px 10px;
  border: 1px solid #d2d2d7;
  border-radius: 6px;
  background-color: #ffffff;
  color: #007aff;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-today-small:hover {
  background-color: #f0f6ff;
}

.add-todo {
  padding: 0 18px 14px;
}

.error-bar {
  background-color: #ffe5e5;
  color: #ff3b30;
  font-size: 12px;
  padding: 6px 10px;
  border-radius: 6px;
  margin-bottom: 8px;
  word-break: break-all;
}

.type-toggle {
  display: flex;
  border: 1px solid #e5e5e7;
  border-radius: 8px;
  margin-bottom: 8px;
  overflow: hidden;
  background-color: #f5f5f7;
  padding: 2px;
  gap: 2px;
}

.type-btn {
  flex: 1;
  padding: 5px;
  border: none;
  background-color: transparent;
  color: #86868b;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.type-btn.active {
  background-color: #ffffff;
  color: #1d1d1f;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.input-row {
  display: flex;
  gap: 8px;
}

.todo-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #e5e5e7;
  border-radius: 8px;
  font-size: 13px;
  outline: none;
  background-color: #ffffff;
  transition: all 0.2s ease;
}

.todo-input:focus {
  border-color: #007aff;
  box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.12);
}

.btn-add {
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 8px;
  background-color: #007aff;
  color: #ffffff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
  flex-shrink: 0;
}

.btn-add:hover:not(:disabled) {
  background-color: #0066d6;
}

.btn-add:disabled {
  background-color: #c7c7cc;
  cursor: not-allowed;
}

.todo-sections {
  flex: 1;
  overflow-y: auto;
  padding: 0 18px 18px;
}

.todo-section {
  margin-bottom: 14px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: #86868b;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-count {
  font-size: 11px;
  color: #86868b;
  background-color: #ececee;
  padding: 1px 8px;
  border-radius: 10px;
  font-weight: 500;
}

.todo-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.empty-state {
  padding: 14px;
  text-align: center;
  font-size: 12px;
  color: #c7c7cc;
}
</style>
