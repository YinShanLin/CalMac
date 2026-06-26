// 中国法定节假日和调休数据 (2024-2025)
// 格式: MM-DD 为法定节假日, WORK:MM-DD 为调休工作日

export const HOLIDAYS_2024: Record<string, string> = {
  "01-01": "元旦",
  "02-10": "春节",
  "02-11": "春节",
  "02-12": "春节",
  "02-13": "春节",
  "02-14": "春节",
  "02-15": "春节",
  "02-16": "春节",
  "02-17": "春节",
  "04-04": "清明节",
  "04-05": "清明节",
  "04-06": "清明节",
  "05-01": "劳动节",
  "05-02": "劳动节",
  "05-03": "劳动节",
  "05-04": "劳动节",
  "05-05": "劳动节",
  "06-08": "端午节",
  "06-09": "端午节",
  "06-10": "端午节",
  "09-15": "中秋节",
  "09-16": "中秋节",
  "09-17": "中秋节",
  "10-01": "国庆节",
  "10-02": "国庆节",
  "10-03": "国庆节",
  "10-04": "国庆节",
  "10-05": "国庆节",
  "10-06": "国庆节",
  "10-07": "国庆节",
};

export const HOLIDAYS_2025: Record<string, string> = {
  "01-01": "元旦",
  "01-28": "春节",
  "01-29": "春节",
  "01-30": "春节",
  "01-31": "春节",
  "02-01": "春节",
  "02-02": "春节",
  "02-03": "春节",
  "02-04": "春节",
  "04-04": "清明节",
  "04-05": "清明节",
  "04-06": "清明节",
  "05-01": "劳动节",
  "05-02": "劳动节",
  "05-03": "劳动节",
  "05-04": "劳动节",
  "05-05": "劳动节",
  "05-31": "端午节",
  "06-01": "端午节",
  "06-02": "端午节",
  "10-01": "国庆节",
  "10-02": "国庆节",
  "10-03": "国庆节",
  "10-04": "中秋节",
  "10-05": "国庆节",
  "10-06": "国庆节",
  "10-07": "国庆节",
  "10-08": "国庆节",
};

// 2026 年放假安排（参考，以国务院公告为准）
export const HOLIDAYS_2026: Record<string, string> = {
  "01-01": "元旦",
  "02-16": "春节",
  "02-17": "春节",
  "02-18": "春节",
  "02-19": "春节",
  "02-20": "春节",
  "02-21": "春节",
  "02-22": "春节",
  "04-04": "清明节",
  "04-05": "清明节",
  "04-06": "清明节",
  "05-01": "劳动节",
  "05-02": "劳动节",
  "05-03": "劳动节",
  "05-04": "劳动节",
  "05-05": "劳动节",
  "06-19": "端午节",
  "06-20": "端午节",
  "06-21": "端午节",
  "09-25": "中秋节",
  "09-26": "中秋节",
  "09-27": "中秋节",
  "10-01": "国庆节",
  "10-02": "国庆节",
  "10-03": "国庆节",
  "10-04": "国庆节",
  "10-05": "国庆节",
  "10-06": "国庆节",
  "10-07": "国庆节",
};

// 调休工作日（周末需要上班的日子）
export const WORK_DAYS_2024: Record<string, boolean> = {
  "02-04": true,
  "02-18": true,
  "04-07": true,
  "04-28": true,
  "05-11": true,
  "09-14": true,
  "09-29": true,
  "10-12": true,
};

export const WORK_DAYS_2025: Record<string, boolean> = {
  "01-26": true,
  "02-08": true,
  "04-27": true,
  "05-04": true,
  "09-28": true,
  "10-11": true,
};

export const WORK_DAYS_2026: Record<string, boolean> = {
  "02-15": true,
  "02-28": true,
  "04-26": true,
  "05-09": true,
  "09-27": true,
  "10-10": true,
};

const HOLIDAYS_BY_YEAR: Record<number, Record<string, string>> = {
  2024: HOLIDAYS_2024,
  2025: HOLIDAYS_2025,
  2026: HOLIDAYS_2026,
};

const WORK_DAYS_BY_YEAR: Record<number, Record<string, boolean>> = {
  2024: WORK_DAYS_2024,
  2025: WORK_DAYS_2025,
  2026: WORK_DAYS_2026,
};

export function isHoliday(date: Date): string | null {
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const key = `${month}-${day}`;
  const year = date.getFullYear();
  return HOLIDAYS_BY_YEAR[year]?.[key] || null;
}

export function isWorkDay(date: Date): boolean {
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const key = `${month}-${day}`;
  const year = date.getFullYear();
  return WORK_DAYS_BY_YEAR[year]?.[key] || false;
}

export function isWeekend(date: Date): boolean {
  const day = date.getDay();
  return day === 0 || day === 6;
}
