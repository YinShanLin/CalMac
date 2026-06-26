<script setup lang="ts">
import { computed } from "vue";
import { format } from "date-fns";
import type { Event } from "@/types/event";

const props = defineProps<{
  event: Event;
}>();

const eventTime = computed(() => {
  if (props.event.all_day) return "全天";
  return format(new Date(props.event.start_time), "HH:mm");
});

const eventColor = computed(() => props.event.color || "#007aff");
</script>

<template>
  <div class="event-item" :style="{ '--event-color': eventColor }">
    <span class="event-bar" />
    <span class="event-time">{{ eventTime }}</span>
    <span class="event-title">{{ event.title }}</span>
  </div>
</template>

<style scoped>
.event-item {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 2px 6px 2px 0;
  border-radius: 5px;
  font-size: 11px;
  overflow: hidden;
  cursor: pointer;
  background-color: color-mix(in srgb, var(--event-color) 14%, transparent);
  transition: background-color 0.15s ease;
}

.event-item:hover {
  background-color: color-mix(in srgb, var(--event-color) 22%, transparent);
}

.event-bar {
  flex-shrink: 0;
  width: 3px;
  align-self: stretch;
  background-color: var(--event-color);
  border-radius: 2px;
}

.event-time {
  font-weight: 600;
  color: var(--event-color);
  white-space: nowrap;
  margin-left: 5px;
}

.event-title {
  color: #1d1d1f;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 500;
}
</style>
