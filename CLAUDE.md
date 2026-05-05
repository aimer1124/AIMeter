# AIMeter — AI Development Guide

## Project Overview

AIMeter is a cross-platform menu bar app for developers to monitor AI coding tool usage and costs.
Built with Tauri 2 (Rust backend) + React/TypeScript (frontend) + Vite.

## Commands

- `npm run tauri dev` — Start development (frontend + backend hot reload)
- `npm run tauri build` — Production build
- `npm run dev` — Frontend only dev server (port 1420)
- `npm run build` — Frontend only build
- `cargo test --manifest-path src-tauri/Cargo.toml` — Run Rust tests
- `npx tsc --noEmit` — TypeScript type check

## Architecture

```
src-tauri/src/          Rust backend
├── main.rs             App setup, tray icon, command registration
├── providers.rs        Provider config CRUD (stored in tauri-plugin-store)
├── usage.rs            Usage data aggregation from provider APIs
└── ai/
    ├── mod.rs          AI analysis module entry
    ├── insights.rs     Cost pattern analysis, anomaly detection
    └── predictions.rs  Budget exhaustion forecasting

src/                    React frontend
├── components/         UI components
│   ├── Dashboard.tsx   Main overview
│   ├── UsageCard.tsx   Per-provider card
│   ├── Settings.tsx    Configuration
│   └── ai/            AI-powered components
│       ├── InsightsPanel.tsx   Smart analysis display
│       └── PredictionChart.tsx Budget forecast visualization
├── hooks/
│   ├── useUsage.ts    Usage data fetching hook
│   └── useInsights.ts AI insights hook
└── styles/global.css   Styling
```

## Conventions

- Rust: snake_case, derive Serialize/Deserialize on all transfer types
- TypeScript: camelCase for variables, PascalCase for components
- CSS: BEM-ish class names, CSS variables for theming
- All provider API calls go through Rust backend (security: keys never touch frontend)
- AI features are non-blocking — app works fully without network access

## Key Design Decisions

- Tray-first: app lives in system tray, popup panel for quick glance
- Offline-capable: cached usage data shown even when APIs are unreachable
- Privacy: API keys stored locally via tauri-plugin-store, never transmitted
- AI insights run locally using heuristic algorithms (no external AI API calls for analysis)
