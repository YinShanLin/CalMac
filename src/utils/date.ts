// 本地时区安全的日期工具函数
// 避免 toISOString() 转 UTC 导致跨天错位

export function toLocalDateKey(d: Date): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}

export function todayKey(): string {
  return toLocalDateKey(new Date());
}

// 将本地日期键 + 时间拼成 ISO 字符串（用于存储 start_time/end_time）
export function combineLocalDateTime(dateKey: string, time: string): string {
  return new Date(`${dateKey}T${time}`).toISOString();
}

// 从 ISO 时间字符串取本地日期键（用于按日分组）
export function isoToLocalDateKey(iso: string): string {
  return toLocalDateKey(new Date(iso));
}

export function isTodayKey(key: string): boolean {
  return key === todayKey();
}
