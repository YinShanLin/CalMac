<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { format } from "date-fns";
import type { Event, CreateEventRequest } from "@/types/event";
import { useCalendarStore } from "@/stores/calendar";

const props = defineProps<{
  modelValue: boolean;
  event: Event | null;
  defaultDate: Date | null;
}>();

const emit = defineEmits<{
  "update:modelValue": [v: boolean];
  saved: [];
}>();

const calendarStore = useCalendarStore();

const title = ref("");
const description = ref("");
const allDay = ref(false);
const startDate = ref("");
const startTime = ref("09:00");
const endDate = ref("");
const endTime = ref("10:00");
const color = ref("#007aff");
const remindMinutes = ref<number | null>(null);

const isEdit = computed(() => !!props.event);
const canSave = computed(() => title.value.trim().length > 0);

const colors = [
  "#007aff", "#ff3b30", "#ff9500", "#34c759",
  "#5856d6", "#af52de", "#ff2d55", "#a2845e",
];

const remindOptions = [
  { label: "无提醒", value: null },
  { label: "5 分钟前", value: 5 },
  { label: "10 分钟前", value: 10 },
  { label: "15 分钟前", value: 15 },
  { label: "30 分钟前", value: 30 },
  { label: "1 小时前", value: 60 },
];

function toLocalInput(iso: string): { date: string; time: string } {
  const d = new Date(iso);
  return { date: format(d, "yyyy-MM-dd"), time: format(d, "HH:mm") };
}

watch(
  () => props.modelValue,
  (open) => {
    if (!open) return;
    if (props.event) {
      title.value = props.event.title;
      description.value = props.event.description || "";
      allDay.value = props.event.all_day;
      const s = toLocalInput(props.event.start_time);
      const e = toLocalInput(props.event.end_time);
      startDate.value = s.date;
      startTime.value = s.time;
      endDate.value = e.date;
      endTime.value = e.time;
      color.value = props.event.color || "#007aff";
      remindMinutes.value = props.event.remind_minutes ?? null;
    } else {
      const base = props.defaultDate || new Date();
      title.value = "";
      description.value = "";
      allDay.value = false;
      startDate.value = format(base, "yyyy-MM-dd");
      endDate.value = format(base, "yyyy-MM-dd");
      startTime.value = "09:00";
      endTime.value = "10:00";
      color.value = "#007aff";
      remindMinutes.value = null;
    }
  }
);

function close() {
  emit("update:modelValue", false);
}

function buildIso(date: string, time: string): string {
  return new Date(`${date}T${time}`).toISOString();
}

async function submit() {
  if (!canSave.value) return;
  const start_time = buildIso(startDate.value, allDay.value ? "00:00" : startTime.value);
  const end_time = buildIso(endDate.value, allDay.value ? "23:59" : endTime.value);

  if (isEdit.value && props.event) {
    await calendarStore.updateEvent({
      id: props.event.id,
      title: title.value.trim(),
      description: description.value || null,
      start_time,
      end_time,
      all_day: allDay.value,
      color: color.value,
      remind_minutes: remindMinutes.value,
    });
  } else {
    const request: CreateEventRequest = {
      title: title.value.trim(),
      description: description.value || null,
      start_time,
      end_time,
      all_day: allDay.value,
      color: color.value,
      remind_minutes: remindMinutes.value,
    };
    await calendarStore.createEvent(request);
  }
  emit("saved");
  close();
}

async function remove() {
  if (!props.event) return;
  await calendarStore.deleteEvent(props.event.id);
  emit("saved");
  close();
}
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="modelValue" class="overlay" @click.self="close">
        <div class="modal-card">
          <header class="modal-head">
            <h2>{{ isEdit ? "编辑日程" : "新建日程" }}</h2>
            <button class="btn-close" @click="close">
              <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                <path d="M11 3L3 11M3 3L11 11" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
              </svg>
            </button>
          </header>

          <div class="modal-body">
            <div class="color-strip" :style="{ backgroundColor: color }" />

            <input
              v-model="title"
              class="input-title"
              placeholder="标题"
              @keydown.enter="submit"
            />

            <div class="row">
              <label class="switch">
                <input v-model="allDay" type="checkbox" />
                <span class="track"><span class="thumb" /></span>
                <span class="switch-label">全天</span>
              </label>
            </div>

            <div class="datetime-grid">
              <div class="field">
                <span class="field-label">开始</span>
                <div class="field-controls">
                  <input v-model="startDate" type="date" class="input-date" />
                  <input v-if="!allDay" v-model="startTime" type="time" class="input-time" />
                </div>
              </div>
              <div class="field">
                <span class="field-label">结束</span>
                <div class="field-controls">
                  <input v-model="endDate" type="date" class="input-date" />
                  <input v-if="!allDay" v-model="endTime" type="time" class="input-time" />
                </div>
              </div>
            </div>

            <div class="field">
              <span class="field-label">提醒</span>
              <div class="remind-options">
                <button
                  v-for="opt in remindOptions"
                  :key="String(opt.value)"
                  class="chip"
                  :class="{ active: remindMinutes === opt.value }"
                  @click="remindMinutes = opt.value"
                >{{ opt.label }}</button>
              </div>
            </div>

            <div class="field">
              <span class="field-label">颜色</span>
              <div class="color-options">
                <button
                  v-for="c in colors"
                  :key="c"
                  class="color-dot"
                  :class="{ active: color === c }"
                  :style="{ backgroundColor: c }"
                  @click="color = c"
                />
              </div>
            </div>

            <div class="field">
              <span class="field-label">备注</span>
              <textarea v-model="description" class="input-notes" rows="3" placeholder="添加备注" />
            </div>

            <div v-if="calendarStore.error" class="error-bar">{{ calendarStore.error }}</div>
          </div>

          <footer class="modal-foot">
            <button v-if="isEdit" class="btn-danger" @click="remove">删除</button>
            <div class="foot-right">
              <button class="btn-ghost" @click="close">取消</button>
              <button class="btn-primary" :disabled="!canSave || calendarStore.loading" @click="submit">
                {{ calendarStore.loading ? "保存中..." : "保存" }}
              </button>
            </div>
          </footer>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.32);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-card {
  width: 440px;
  max-height: 88vh;
  background-color: #ffffff;
  border-radius: 16px;
  box-shadow: 0 24px 70px rgba(0, 0, 0, 0.22);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.modal-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 22px;
}

.modal-head h2 {
  font-size: 16px;
  font-weight: 600;
  color: #1d1d1f;
}

.btn-close {
  width: 26px;
  height: 26px;
  border: none;
  border-radius: 50%;
  background-color: #f5f5f7;
  color: #86868b;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.btn-close:hover {
  background-color: #e8e8ed;
  color: #1d1d1f;
}

.modal-body {
  padding: 4px 22px 18px;
  overflow-y: auto;
}

.color-strip {
  height: 3px;
  border-radius: 2px;
  margin-bottom: 16px;
}

.input-title {
  width: 100%;
  padding: 10px 0;
  border: none;
  border-bottom: 1px solid #e5e5e5;
  font-size: 18px;
  font-weight: 500;
  color: #1d1d1f;
  outline: none;
  background: transparent;
}

.input-title:focus {
  border-bottom-color: #007aff;
}

.row {
  margin-top: 14px;
}

.switch {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.switch input {
  display: none;
}

.track {
  width: 38px;
  height: 22px;
  border-radius: 11px;
  background-color: #d2d2d7;
  position: relative;
  transition: background-color 0.2s ease;
}

.thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background-color: #ffffff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  transition: transform 0.2s ease;
}

.switch input:checked + .track {
  background-color: #34c759;
}

.switch input:checked + .track .thumb {
  transform: translateX(16px);
}

.switch-label {
  font-size: 13px;
  color: #1d1d1f;
}

.datetime-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-top: 14px;
}

.field {
  margin-top: 14px;
}

.field-label {
  display: block;
  font-size: 12px;
  font-weight: 600;
  color: #86868b;
  margin-bottom: 6px;
}

.field-controls {
  display: flex;
  gap: 8px;
}

.input-date,
.input-time {
  flex: 1;
  padding: 8px 10px;
  border: 1px solid #d2d2d7;
  border-radius: 8px;
  font-size: 13px;
  color: #1d1d1f;
  outline: none;
  background-color: #ffffff;
  transition: border-color 0.2s ease;
}

.input-date:focus,
.input-time:focus {
  border-color: #007aff;
}

.remind-options,
.color-options {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.chip {
  padding: 5px 11px;
  border: 1px solid #d2d2d7;
  border-radius: 14px;
  background-color: #ffffff;
  color: #1d1d1f;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.chip:hover {
  background-color: #f5f5f7;
}

.chip.active {
  background-color: #007aff;
  border-color: #007aff;
  color: #ffffff;
}

.color-dot {
  width: 22px;
  height: 22px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
  padding: 0;
  transition: transform 0.15s ease;
}

.color-dot:hover {
  transform: scale(1.12);
}

.color-dot.active {
  border-color: #1d1d1f;
  box-shadow: 0 0 0 2px #ffffff inset;
}

.input-notes {
  width: 100%;
  padding: 10px;
  border: 1px solid #d2d2d7;
  border-radius: 8px;
  font-size: 13px;
  color: #1d1d1f;
  outline: none;
  resize: vertical;
  font-family: inherit;
  transition: border-color 0.2s ease;
}

.input-notes:focus {
  border-color: #007aff;
}

.error-bar {
  margin-top: 12px;
  padding: 8px 10px;
  background-color: #ffe5e5;
  color: #ff3b30;
  font-size: 12px;
  border-radius: 8px;
  word-break: break-all;
}

.modal-foot {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 22px;
  border-top: 1px solid #f0f0f2;
}

.foot-right {
  display: flex;
  gap: 8px;
  margin-left: auto;
}

.btn-ghost {
  padding: 8px 16px;
  border: 1px solid transparent;
  border-radius: 8px;
  background-color: transparent;
  color: #1d1d1f;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.btn-ghost:hover {
  background-color: #f5f5f7;
}

.btn-primary {
  padding: 8px 18px;
  border: none;
  border-radius: 8px;
  background-color: #007aff;
  color: #ffffff;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.btn-primary:hover:not(:disabled) {
  background-color: #0066d6;
}

.btn-primary:disabled {
  background-color: #b0b0b5;
  cursor: not-allowed;
}

.btn-danger {
  padding: 8px 14px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: #ff3b30;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.btn-danger:hover {
  background-color: #ffe5e5;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-active .modal-card,
.modal-leave-active .modal-card {
  transition: transform 0.25s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-card,
.modal-leave-to .modal-card {
  transform: scale(0.96) translateY(8px);
}
</style>
