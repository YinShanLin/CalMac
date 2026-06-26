import { defineStore } from "pinia";
import { ref, computed } from "vue";
import {
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
} from "date-fns";
import type { Event, CreateEventRequest, UpdateEventRequest } from "@/types/event";
import { invoke } from "@tauri-apps/api/core";
import { toLocalDateKey, isoToLocalDateKey } from "@/utils/date";

export const useCalendarStore = defineStore("calendar", () => {
  const events = ref<Event[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const weekStartsOnMonday = ref(localStorage.getItem("weekStartsOnMonday") !== "false");

  function toggleWeekStart() {
    weekStartsOnMonday.value = !weekStartsOnMonday.value;
    localStorage.setItem("weekStartsOnMonday", String(weekStartsOnMonday.value));
  }

  const eventsByDate = computed(() => {
    const map = new Map<string, Event[]>();
    for (const e of events.value) {
      const key = isoToLocalDateKey(e.start_time);
      if (!map.has(key)) map.set(key, []);
      map.get(key)!.push(e);
    }
    return map;
  });

  async function loadMonth(date: Date) {
    loading.value = true;
    error.value = null;
    try {
      const weekStartsOn = weekStartsOnMonday.value ? 1 : 0;
      const calStart = startOfWeek(startOfMonth(date), { weekStartsOn });
      const calEnd = endOfWeek(endOfMonth(date), { weekStartsOn });
      const start = calStart.toISOString();
      const end = addDays(calEnd, 1).toISOString();
      events.value = await invoke<Event[]>("get_events_by_range", { start, end });
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function loadEvents() {
    loading.value = true;
    error.value = null;
    try {
      events.value = await invoke<Event[]>("get_events");
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createEvent(request: CreateEventRequest): Promise<Event | null> {
    loading.value = true;
    error.value = null;
    try {
      const event = await invoke<Event>("create_event", { request });
      events.value.push(event);
      return event;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function updateEvent(request: UpdateEventRequest): Promise<Event | null> {
    loading.value = true;
    error.value = null;
    try {
      const event = await invoke<Event>("update_event", { request });
      const index = events.value.findIndex((e) => e.id === event.id);
      if (index !== -1) {
        events.value[index] = event;
      }
      return event;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function deleteEvent(eventId: string): Promise<boolean> {
    loading.value = true;
    error.value = null;
    try {
      await invoke("delete_event", { eventId });
      events.value = events.value.filter((e) => e.id !== eventId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    } finally {
      loading.value = false;
    }
  }

  function getEventsForDate(date: Date): Event[] {
    return eventsByDate.value.get(toLocalDateKey(date)) ?? [];
  }

  return {
    events,
    loading,
    error,
    weekStartsOnMonday,
    eventsByDate,
    toggleWeekStart,
    loadMonth,
    loadEvents,
    createEvent,
    updateEvent,
    deleteEvent,
    getEventsForDate,
  };
});
