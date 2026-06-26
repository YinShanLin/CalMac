<script setup lang="ts">
import { ref, nextTick } from "vue";
import type { Todo } from "@/types/event";
import { useTodoStore } from "@/stores/todo";

const props = defineProps<{
  todo: Todo;
}>();

const todoStore = useTodoStore();
const isEditing = ref(false);
const editTitle = ref(props.todo.title);
const editInput = ref<HTMLInputElement | null>(null);

async function toggleComplete() {
  await todoStore.updateTodo(props.todo.id, {
    completed: !props.todo.completed,
  });
}

async function startEdit() {
  editTitle.value = props.todo.title;
  isEditing.value = true;
  await nextTick();
  editInput.value?.focus();
}

async function saveEdit() {
  if (editTitle.value.trim() && editTitle.value !== props.todo.title) {
    await todoStore.updateTodo(props.todo.id, {
      title: editTitle.value.trim(),
    });
  }
  isEditing.value = false;
}

function cancelEdit() {
  isEditing.value = false;
  editTitle.value = props.todo.title;
}

async function deleteTodo() {
  await todoStore.deleteTodo(props.todo.id);
}

async function cyclePriority() {
  const priorities: Array<"high" | "medium" | "low" | null> = [null, "high", "medium", "low"];
  const currentIndex = priorities.indexOf(props.todo.priority ?? null);
  const nextPriority = priorities[(currentIndex + 1) % priorities.length];
  await todoStore.updateTodo(props.todo.id, {
    priority: nextPriority,
  });
}

function getPriorityColor(priority?: string | null) {
  switch (priority) {
    case "high":
      return "#ff3b30";
    case "medium":
      return "#ff9500";
    case "low":
      return "#34c759";
    default:
      return "#d2d2d7";
  }
}
</script>

<template>
  <div class="todo-item" :class="{ completed: todo.completed }">
    <button class="checkbox" :class="{ checked: todo.completed }" @click="toggleComplete">
      <svg v-if="todo.completed" width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M10 3L4.5 8.5L2 6" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </button>

    <button
      class="priority-dot"
      :style="{ backgroundColor: getPriorityColor(todo.priority) }"
      :title="todo.priority ? `优先级：${todo.priority}` : '点击设置优先级'"
      @click="cyclePriority"
    />

    <div class="todo-content" v-if="!isEditing">
      <span class="todo-title" @dblclick="startEdit">{{ todo.title }}</span>
    </div>

    <input
      v-else
      ref="editInput"
      v-model="editTitle"
      class="edit-input"
      @blur="saveEdit"
      @keydown.enter="saveEdit"
      @keydown.escape="cancelEdit"
    />

    <button class="btn-delete" @click="deleteTodo" aria-label="删除">
      <svg width="13" height="13" viewBox="0 0 14 14" fill="none">
        <path d="M10.5 3.5L3.5 10.5M3.5 3.5L10.5 10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
      </svg>
    </button>
  </div>
</template>

<style scoped>
.todo-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 7px 10px;
  border-radius: 8px;
  transition: background-color 0.15s ease;
}

.todo-item:hover {
  background-color: #ffffff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.todo-item:hover .btn-delete {
  opacity: 1;
}

.checkbox {
  width: 18px;
  height: 18px;
  border: 2px solid #d2d2d7;
  border-radius: 50%;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
  color: transparent;
  padding: 0;
}

.checkbox:hover {
  border-color: #007aff;
}

.checkbox.checked {
  background-color: #34c759;
  border-color: #34c759;
  color: #ffffff;
}

.priority-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  flex-shrink: 0;
  transition: transform 0.15s ease;
  padding: 0;
}

.priority-dot:hover {
  transform: scale(1.25);
}

.todo-content {
  flex: 1;
  min-width: 0;
}

.todo-title {
  font-size: 13px;
  color: #1d1d1f;
  cursor: default;
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.completed .todo-title {
  text-decoration: line-through;
  color: #aeaeb2;
}

.edit-input {
  flex: 1;
  padding: 4px 8px;
  border: 1px solid #007aff;
  border-radius: 6px;
  font-size: 13px;
  outline: none;
  box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.12);
}

.btn-delete {
  opacity: 0;
  width: 22px;
  height: 22px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #86868b;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.btn-delete:hover {
  background-color: #ff3b30;
  color: #ffffff;
}
</style>
