export interface Event {
  id: string;
  title: string;
  description?: string;
  start_time: string;
  end_time: string;
  all_day: boolean;
  color?: string;
  remind_minutes?: number | null;
  created_at: string;
  updated_at: string;
}

export interface CreateEventRequest {
  title: string;
  description: string | null;
  start_time: string;
  end_time: string;
  all_day: boolean;
  color: string;
  remind_minutes: number | null;
}

export interface UpdateEventRequest {
  id: string;
  title: string | null;
  description: string | null;
  start_time: string | null;
  end_time: string | null;
  all_day: boolean | null;
  color: string | null;
  remind_minutes: number | null;
}

export interface Todo {
  id: string;
  title: string;
  completed: boolean;
  todo_type: "global" | "daily";
  date?: string;
  priority?: "high" | "medium" | "low";
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface CreateTodoRequest {
  title: string;
  todo_type: "global" | "daily";
  date: string | null;
  priority: "high" | "medium" | "low" | null;
}

export interface UpdateTodoRequest {
  id: string;
  title: string | null;
  completed: boolean | null;
  priority: "high" | "medium" | "low" | null;
  sort_order: number | null;
}
