<script setup lang="ts">
import { ref, computed, nextTick } from "vue";
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
  isSameMonth,
  isSameDay,
} from "date-fns";
import { useCalendarStore } from "@/stores/calendar";
import { useTodoStore } from "@/stores/todo";
import { isHoliday, isWorkDay, isWeekend } from "@/utils/holidays";
import EventItem from "./EventItem.vue";
import EventForm from "./EventForm.vue";
import type { Event } from "@/types/event";

const props = defineProps<{
  currentDate: Date;
}>();

const calendarStore = useCalendarStore();
const todoStore = useTodoStore();

const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  date: null as Date | null,
});
const menuRef = ref<HTMLElement | null>(null);

const showEventForm = ref(false);
const editingEvent = ref<Event | null>(null);
const formDefaultDate = ref<Date | null>(null);

const weekDays = computed(() => {
  if (calendarStore.weekStartsOnMonday) {
    return ["一", "二", "三", "四", "五", "六", "日"];
  }
  return ["日", "一", "二", "三", "四", "五", "六"];
});

const calendarDays = computed(() => {
  const monthStart = startOfMonth(props.currentDate);
  const monthEnd = endOfMonth(props.currentDate);
  const weekStartsOn = calendarStore.weekStartsOnMonday ? 1 : 0;
  const calStart = startOfWeek(monthStart, { weekStartsOn });
  const calEnd = endOfWeek(monthEnd, { weekStartsOn });

  const days = [];
  let day = calStart;

  while (day <= calEnd) {
    const holiday = isHoliday(day);
    const workDay = isWorkDay(day);
    const weekend = isWeekend(day);

    days.push({
      date: day,
      isCurrentMonth: isSameMonth(day, props.currentDate),
      isToday: isSameDay(day, new Date()),
      events: calendarStore.getEventsForDate(day),
      isHoliday: !!holiday,
      holidayName: holiday,
      isWorkDay: workDay,
      isWeekend: weekend && !workDay,
    });
    day = addDays(day, 1);
  }

  return days;
});

function getDayClass(day: any) {
  return {
    "other-month": !day.isCurrentMonth,
    today: day.isToday,
    holiday: day.isHoliday && day.isCurrentMonth,
    "work-day": day.isWorkDay && day.isCurrentMonth,
    weekend: day.isWeekend && day.isCurrentMonth,
  };
}

function getDayNumberClass(day: any) {
  return {
    today: day.isToday,
    "holiday-text": day.isHoliday && !day.isToday,
    "work-day-text": day.isWorkDay && !day.isToday,
    "weekend-text": day.isWeekend && !day.isToday,
  };
}

async function handleContextMenu(e: MouseEvent, day: any) {
  e.preventDefault();
  contextMenu.value = {
    show: true,
    x: e.clientX,
    y: e.clientY,
    date: day.date,
  };
  await nextTick();
  adjustMenuPosition();
}

function adjustMenuPosition() {
  const el = menuRef.value;
  if (!el) return;
  const rect = el.getBoundingClientRect();
  let x = contextMenu.value.x;
  let y = contextMenu.value.y;
  if (x + rect.width > window.innerWidth - 8) {
    x = window.innerWidth - rect.width - 8;
  }
  if (y + rect.height > window.innerHeight - 8) {
    y = window.innerHeight - rect.height - 8;
  }
  contextMenu.value.x = x;
  contextMenu.value.y = y;
}

function closeContextMenu() {
  contextMenu.value.show = false;
}

function openCreateEvent(day: any) {
  editingEvent.value = null;
  formDefaultDate.value = day.date;
  showEventForm.value = true;
  closeContextMenu();
}

function openCreateEventFromMenu() {
  if (!contextMenu.value.date) return;
  openCreateEvent({ date: contextMenu.value.date });
}

function editEvent(event: Event) {
  editingEvent.value = event;
  formDefaultDate.value = null;
  showEventForm.value = true;
}

async function createDailyTodo() {
  if (!contextMenu.value.date) return;
  const dateStr = format(contextMenu.value.date, "yyyy-MM-dd");
  todoStore.setSelectedDate(dateStr);
  await todoStore.createTodo("新待办", "daily", dateStr);
  closeContextMenu();
}

async function createGlobalTodo() {
  await todoStore.createTodo("新待办", "global");
  closeContextMenu();
}
</script>

<template>
  <div class="month-view" @click="closeContextMenu">
    <div class="week-header">
      <div
        v-for="(day, index) in weekDays"
        :key="day"
        class="week-day-name"
        :class="{
          weekend: calendarStore.weekStartsOnMonday ? index >= 5 : index === 0 || index === 6,
        }"
      >
        {{ day }}
      </div>
    </div>
    <div class="calendar-grid">
      <div
        v-for="(day, index) in calendarDays"
        :key="index"
        class="calendar-cell"
        :class="getDayClass(day)"
        @click="openCreateEvent(day)"
        @contextmenu="handleContextMenu($event, day)"
      >
        <div class="cell-header">
          <span class="day-number" :class="getDayNumberClass(day)">
            {{ format(day.date, "d") }}
          </span>
          <span v-if="day.isHoliday && day.isCurrentMonth" class="holiday-badge">
            {{ day.holidayName }}
          </span>
        </div>
        <div class="cell-events">
          <EventItem
            v-for="event in day.events.slice(0, 3)"
            :key="event.id"
            :event="event"
            @click.stop="editEvent(event)"
          />
          <div v-if="day.events.length > 3" class="more-events">
            +{{ day.events.length - 3 }} 更多
          </div>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <div
        v-if="contextMenu.show"
        ref="menuRef"
        class="context-menu"
        :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
        @click.stop
      >
        <div class="context-menu-item" @click="openCreateEventFromMenu">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
            <rect x="2" y="3" width="10" height="9" rx="1.5" stroke="currentColor" stroke-width="1.3" />
            <path d="M2 6H12" stroke="currentColor" stroke-width="1.3" />
            <path d="M7 8.5V11M5.5 9.75H8.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" />
          </svg>
          新建日程
        </div>
        <div class="context-menu-item" @click="createDailyTodo">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
            <path d="M7 3V11M3 7H11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
          新建当日待办
        </div>
        <div class="context-menu-item" @click="createGlobalTodo">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
            <path d="M7 3V11M3 7H11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
          新建全局待办
        </div>
      </div>
    </Teleport>

    <EventForm
      v-model="showEventForm"
      :event="editingEvent"
      :default-date="formDefaultDate"
    />
  </div>
</template>

<style scoped>
.month-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
}

.week-header {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  border-bottom: 1px solid #ececee;
}

.week-day-name {
  padding: 10px;
  text-align: center;
  font-size: 11px;
  font-weight: 600;
  color: #86868b;
  letter-spacing: 0.5px;
}

.week-day-name.weekend {
  color: #ff3b30;
}

.calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  grid-template-rows: repeat(6, 1fr);
  flex: 1;
}

.calendar-cell {
  border-right: 1px solid #f0f0f2;
  border-bottom: 1px solid #f0f0f2;
  padding: 6px 6px 4px;
  min-height: 96px;
  cursor: pointer;
  transition: background-color 0.15s ease;
  position: relative;
}

.calendar-cell:hover {
  background-color: #fafafc;
}

.calendar-cell.other-month {
  background-color: #fbfbfd;
}

.calendar-cell.other-month .day-number {
  color: #c7c7cc;
}

.calendar-cell.today {
  background-color: #eef6ff;
}

.calendar-cell.holiday {
  background-color: #fff5f5;
}

.calendar-cell.work-day {
  background-color: #fffaeb;
}

.calendar-cell.weekend {
  background-color: #fafafc;
}

.cell-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 6px;
  gap: 3px;
}

.day-number {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  font-size: 13px;
  font-weight: 500;
  color: #1d1d1f;
  border-radius: 50%;
  transition: all 0.15s ease;
}

.calendar-cell:hover .day-number:not(.today) {
  background-color: #ececee;
}

.day-number.today {
  background-color: #ff3b30;
  color: #ffffff;
  font-weight: 600;
}

.day-number.holiday-text {
  color: #ff3b30;
}

.day-number.work-day-text {
  color: #ff9500;
}

.day-number.weekend-text {
  color: #ff3b30;
}

.holiday-badge {
  font-size: 9px;
  color: #ff3b30;
  background-color: #ffe5e5;
  padding: 1px 5px;
  border-radius: 4px;
  white-space: nowrap;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 500;
}

.cell-events {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.more-events {
  font-size: 11px;
  color: #86868b;
  padding: 1px 4px;
  font-weight: 500;
}

.context-menu {
  position: fixed;
  background-color: rgba(255, 255, 255, 0.96);
  backdrop-filter: blur(20px);
  border: 0.5px solid rgba(0, 0, 0, 0.12);
  border-radius: 10px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.18);
  padding: 5px;
  z-index: 1100;
  min-width: 168px;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 7px 10px;
  font-size: 13px;
  color: #1d1d1f;
  cursor: pointer;
  border-radius: 6px;
  transition: background-color 0.12s ease;
}

.context-menu-item:hover {
  background-color: #007aff;
  color: #ffffff;
}
</style>
