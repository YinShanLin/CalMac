import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Todo, CreateTodoRequest, UpdateTodoRequest } from "@/types/event";
import { invoke } from "@tauri-apps/api/core";
import { todayKey } from "@/utils/date";

export const useTodoStore = defineStore("todo", () => {
  const todos = ref<Todo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const selectedDate = ref<string>(todayKey());

  const globalTodos = computed(() =>
    todos.value
      .filter((t) => t.todo_type === "global")
      .sort((a, b) => a.sort_order - b.sort_order)
  );

  const dailyTodos = computed(() =>
    todos.value
      .filter((t) => t.todo_type === "daily" && t.date === selectedDate.value)
      .sort((a, b) => a.sort_order - b.sort_order)
  );

  async function loadTodos() {
    loading.value = true;
    error.value = null;
    try {
      todos.value = await invoke<Todo[]>("get_todos");
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function createTodo(
    title: string,
    todo_type: "global" | "daily",
    date?: string,
    priority?: "high" | "medium" | "low"
  ): Promise<Todo | null> {
    loading.value = true;
    error.value = null;
    try {
      const request: CreateTodoRequest = {
        title,
        todo_type,
        date: date || null,
        priority: priority || null,
      };
      const todo = await invoke<Todo>("create_todo", { request });
      todos.value.push(todo);
      return todo;
    } catch (e) {
      error.value = String(e);
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function updateTodo(
    id: string,
    options: {
      title?: string;
      completed?: boolean;
      priority?: "high" | "medium" | "low" | null;
      sort_order?: number;
    }
  ): Promise<Todo | null> {
    loading.value = true;
    error.value = null;
    try {
      const request: UpdateTodoRequest = {
        id,
        title: options.title ?? null,
        completed: options.completed ?? null,
        priority: options.priority ?? null,
        sort_order: options.sort_order ?? null,
      };
      const todo = await invoke<Todo>("update_todo", { request });
      const index = todos.value.findIndex((t) => t.id === todo.id);
      if (index !== -1) {
        todos.value[index] = todo;
      }
      return todo;
    } catch (e) {
      error.value = String(e);
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function deleteTodo(todoId: string): Promise<boolean> {
    loading.value = true;
    error.value = null;
    try {
      await invoke("delete_todo", { todoId });
      todos.value = todos.value.filter((t) => t.id !== todoId);
      return true;
    } catch (e) {
      error.value = String(e);
      return false;
    } finally {
      loading.value = false;
    }
  }

  function setSelectedDate(date: string) {
    selectedDate.value = date;
  }

  return {
    todos,
    loading,
    error,
    selectedDate,
    globalTodos,
    dailyTodos,
    loadTodos,
    createTodo,
    updateTodo,
    deleteTodo,
    setSelectedDate,
  };
});
