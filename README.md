<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="CalMac" width="128" height="128" />
</p>

<h1 align="center">CalMac - 日历</h1>

<p align="center">
  基于 Tauri 2.0 + Vue 3 的 macOS 日历应用
</p>

<p align="center">
  <a href="releases/日历_0.1.0_aarch64.dmg"><img src="https://img.shields.io/badge/下载-v0.1.0-blue?logo=apple" /></a>
  <img src="https://img.shields.io/badge/Tauri-2.11-ffc131" />
  <img src="https://img.shields.io/badge/Vue-3.5-4fc08d" />
  <img src="https://img.shields.io/badge/TypeScript-6.0-3178c6" />
  <img src="https://img.shields.io/badge/Rust-2021-dea584" />
  <img src="https://img.shields.io/badge/license-MIT-green" />
</p>

## 功能

- **月视图日历** - 支持月历网格展示，今日高亮，月份切换
- **周起始日切换** - 支持从周一开始或周日开始
- **节假日标注** - 内置中国法定节假日和调休数据，用颜色区分：
  - 🔴 红色：周末、法定节假日
  - 🟠 橙色：调休工作日
  - 🏷️ 节假日名称标签（春节、国庆等）
- **日程管理** - 新增、编辑、删除日程事件，支持全天事件和颜色标记
- **待办事项** - 全局待办 + 当日待办，支持：
  - 新建待办（输入框 + 右键菜单）
  - 标记完成 / 删除
  - 双击编辑
  - 优先级标记（高/中/低）
- **本地存储** - 所有数据存储在 SQLite 本地数据库
- **系统托盘** - 支持 macOS 菜单栏图标

## 截图

| 日历月视图 | 待办面板 |
|:---:|:---:|
| ![月视图](screenshots/calendar.png) | ![待办](screenshots/todo.png) |

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + TypeScript 6 |
| 状态管理 | Pinia 2 |
| 日期处理 | date-fns 4 |
| 桌面框架 | Tauri 2.0 (Rust) |
| 数据库 | SQLite (rusqlite) |
| 构建工具 | Vite 8 |
| 图标生成 | Python Pillow |

## 快速开始

### 前置条件

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/) (推荐使用 rustup 安装)
- macOS (目前仅支持 macOS 构建)

### 开发

```bash
# 安装前端依赖
npm install

# 启动开发模式
npm run tauri dev
```

### 构建

```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/dmg/日历_版本号_aarch64.dmg`。

> 如需预构建的 DMG 安装包，请从 [`releases/`](releases/) 目录下载。

## 项目结构

```
CalMac/
├── src/                          # 前端源码
│   ├── App.vue                   # 根组件
│   ├── main.ts                   # 应用入口
│   ├── components/
│   │   ├── Calendar/
│   │   │   ├── MonthView.vue     # 月视图日历
│   │   │   └── EventItem.vue     # 日程事件条目
│   │   ├── Common/
│   │   │   └── Header.vue        # 顶部导航栏
│   │   └── Todo/
│   │       ├── TodoPanel.vue     # 待办右侧面板
│   │       └── TodoItem.vue      # 待办条目
│   ├── stores/
│   │   ├── calendar.ts           # 日历状态
│   │   └── todo.ts               # 待办状态
│   ├── types/
│   │   └── event.ts              # 类型定义
│   └── utils/
│       └── holidays.ts           # 中国节假日数据
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # 应用入口
│   │   ├── commands/
│   │   │   ├── calendar.rs       # 日程 CRUD 命令
│   │   │   └── todo.rs           # 待办 CRUD 命令
│   │   ├── services/
│   │   │   ├── database.rs       # SQLite 初始化
│   │   │   ├── event_service.rs  # 日程业务逻辑
│   │   │   └── todo_service.rs   # 待办业务逻辑
│   │   └── models/
│   │       └── mod.rs            # 数据模型
│   └── tauri.conf.json           # Tauri 配置
└── package.json
```

## 配置

### 窗口设置

编辑 `src-tauri/tauri.conf.json`：

```json
{
  "app": {
    "windows": [{
      "title": "日历",
      "width": 1100,
      "height": 750,
      "minWidth": 900,
      "minHeight": 600
    }]
  }
}
```

### 节假日数据

节假日数据在 `src/utils/holidays.ts` 中维护，支持按年份添加：

```typescript
export const HOLIDAYS_2026: Record<string, string> = {
  "01-01": "元旦",
  // ...
};
```

## 许可

MIT
