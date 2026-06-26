<script setup lang="ts">
import { computed } from "vue";
import { addMonths, subMonths } from "date-fns";
import { useCalendarStore } from "@/stores/calendar";

const props = defineProps<{
  currentDate: Date;
}>();

const emit = defineEmits<{
  "update:date": [date: Date];
}>();

const calendarStore = useCalendarStore();

const displayDate = computed(() => {
  const month = props.currentDate.getMonth() + 1;
  const year = props.currentDate.getFullYear();
  return `${year} 年 ${month} 月`;
});

function goToToday() {
  emit("update:date", new Date());
}

function goToPreviousMonth() {
  emit("update:date", subMonths(props.currentDate, 1));
}

function goToNextMonth() {
  emit("update:date", addMonths(props.currentDate, 1));
}
</script>

<template>
  <header class="header">
    <div class="header-left">
      <button class="btn-today" @click="goToToday">今天</button>
    </div>
    <div class="header-center">
      <button class="btn-nav" @click="goToPreviousMonth" aria-label="上个月">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path d="M8 10L4 6L8 2" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
      <h1 class="display-date">{{ displayDate }}</h1>
      <button class="btn-nav" @click="goToNextMonth" aria-label="下个月">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path d="M4 10L8 6L4 2" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
    </div>
    <div class="header-right">
      <div class="font-size-controls">
        <button class="btn-font" @click="calendarStore.setFontSize(calendarStore.fontSize - 1)" :disabled="calendarStore.fontSize <= 11" title="缩小字体">A−</button>
        <span class="font-size-value">{{ calendarStore.fontSize }}</span>
        <button class="btn-font" @click="calendarStore.setFontSize(calendarStore.fontSize + 1)" :disabled="calendarStore.fontSize >= 18" title="放大字体">A+</button>
      </div>
      <div class="week-start-toggle">
        <button
          class="toggle-btn"
          :class="{ active: !calendarStore.weekStartsOnMonday }"
          @click="calendarStore.weekStartsOnMonday && calendarStore.toggleWeekStart()"
        >日</button>
        <button
          class="toggle-btn"
          :class="{ active: calendarStore.weekStartsOnMonday }"
          @click="!calendarStore.weekStartsOnMonday && calendarStore.toggleWeekStart()"
        >一</button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 22px;
  background-color: rgba(255, 255, 255, 0.72);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid #ececee;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-right {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  align-items: center;
  flex-shrink: 0;
}

.font-size-controls {
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-font {
  width: 26px;
  height: 26px;
  border: 1px solid #d2d2d7;
  border-radius: 6px;
  background-color: #ffffff;
  color: #1d1d1f;
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.btn-font:hover:not(:disabled) {
  background-color: #f5f5f7;
  border-color: #b0b0b5;
}

.btn-font:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}

.font-size-value {
  font-size: 11px;
  font-weight: 500;
  color: #86868b;
  min-width: 18px;
  text-align: center;
}

.btn-today {
  padding: 7px 16px;
  border: 1px solid #d2d2d7;
  border-radius: 8px;
  background-color: #ffffff;
  color: #1d1d1f;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-today:hover {
  background-color: #f5f5f7;
  border-color: #b0b0b5;
}

.header-center {
  display: flex;
  align-items: center;
  gap: 14px;
}

.btn-nav {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: #1d1d1f;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.btn-nav:hover {
  background-color: #f0f0f2;
}

.display-date {
  font-size: 17px;
  font-weight: 600;
  color: #1d1d1f;
  min-width: 130px;
  text-align: center;
  letter-spacing: 0.3px;
}

.week-start-toggle {
  display: flex;
  border: 1px solid #d2d2d7;
  border-radius: 8px;
  overflow: hidden;
  padding: 1px;
  background-color: #f5f5f7;
}

.toggle-btn {
  padding: 5px 12px;
  border: none;
  background-color: transparent;
  color: #86868b;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.toggle-btn.active {
  background-color: #ffffff;
  color: #1d1d1f;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}
</style>
