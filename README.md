# AIMeter

> **100% AI-built** — This project is entirely designed, coded, and maintained by AI.

> **Status: On Hold** — Currently evaluating [CodexBar](https://github.com/steipete/CodexBar) as an alternative. If it doesn't fully meet our needs, development will resume here.

A cross-platform menu bar tool for developers to monitor AI coding tool usage and costs in one place.

## Why AIMeter?

If you use Claude Code, OpenAI Codex, GitHub Copilot, or Cursor — you're likely juggling multiple dashboards to track what you're spending. AIMeter sits in your menu bar and gives you a single glance at all your AI tool usage, with smart alerts before you blow your budget.

## Features

**Core**
- Unified dashboard for Claude Code, OpenAI Codex, GitHub Copilot, Cursor
- Real-time cost tracking, token usage, and request counts
- Per-provider budget limits with system notifications
- System tray integration — always one glance away
- Cross-platform: macOS, Windows, Linux

**AI-Powered Intelligence**
- Spending pattern analysis with anomaly detection
- Budget exhaustion forecasting (EMA + trend-based)
- Cross-provider cost efficiency comparison
- Smart suggestions to optimize your AI tool spend
- 14-day cost forecast with confidence intervals

## AI-First Development

This project maximizes AI involvement at every stage:

| Layer | AI Role |
|-------|---------|
| Architecture & Planning | Claude Code designs the project structure |
| Implementation | All code authored by AI assistants |
| Code Review | AI review on every PR via [claude-code-action](https://github.com/anthropics/claude-code-action) |
| Testing | AI-generated test suites for all modules |
| Changelog | AI-generated release notes from commit history |
| In-App Analytics | AI-powered spending insights, forecasting, and anomaly detection |
| Development Guide | CLAUDE.md enables any AI assistant to contribute immediately |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Framework | [Tauri 2](https://v2.tauri.app/) (Rust) |
| Frontend | React + TypeScript + Vite |
| AI Engine | Rust-native analysis (insights, predictions, anomaly detection) |
| Storage | tauri-plugin-store (local JSON) |
| CI/CD | GitHub Actions + AI code review |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

### Run Tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npx tsc --noEmit
```

## Project Structure

```
├── src/                      # Frontend (React + TypeScript)
│   ├── components/
│   │   ├── Dashboard.tsx     # Main usage overview
│   │   ├── UsageCard.tsx     # Per-provider usage card
│   │   ├── Settings.tsx      # Provider configuration
│   │   └── ai/              # AI-powered UI components
│   │       ├── InsightsPanel.tsx    # Smart insights display
│   │       └── PredictionChart.tsx  # Cost forecast chart
│   ├── hooks/
│   │   ├── useUsage.ts      # Usage data fetching
│   │   └── useInsights.ts   # AI insights integration
│   └── styles/global.css
├── src-tauri/                # Backend (Rust)
│   └── src/
│       ├── main.rs           # App setup, tray, commands
│       ├── providers.rs      # Provider config management
│       ├── usage.rs          # Usage data aggregation
│       └── ai/              # AI analysis engine
│           ├── insights.rs   # Pattern analysis + anomaly detection
│           └── predictions.rs # Budget forecasting
├── .github/workflows/
│   ├── ci.yml               # CI with AI code review
│   └── release.yml          # Auto-release with AI changelog
└── CLAUDE.md                # AI development guide
```

## Provider Support Roadmap

| Provider | Status | API Method |
|----------|--------|-----------|
| Anthropic (Claude Code) | In Progress | Billing API |
| OpenAI (Codex) | In Progress | Usage API |
| GitHub Copilot | Planned | GitHub API |
| Cursor | Planned | TBD |
| Custom providers | Planned | User-configured endpoints |

## Contributing

This is an AI-first project. We encourage contributions via AI tools:

1. Fork the repo
2. Use Claude Code, Copilot, or your AI tool of choice to implement changes
3. Reference `CLAUDE.md` for project conventions
4. Submit a PR — AI will auto-review it
5. Note which AI tools you used in the PR template

## License

MIT
