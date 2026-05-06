# AIMeter Roadmap

> Strategy: Make Claude Code monitoring deep and complete before expanding to other AI tools.

## MVP 1: Claude Code Complete Experience ✅

> Goal: Daily-drivable — install it and never want to uninstall

| Issue | Feature | Description |
|-------|---------|-------------|
| [#1](../../issues/1) | Account type selection | API / Pro / Max with adaptive display and alerts |
| [#2](../../issues/2) | Historical data storage | Daily snapshots, 90-day retention |
| [#3](../../issues/3) | Time period switching | Today / This Week / This Month / All Time |
| [#4](../../issues/4) | Menu bar cost display | API: `$12.50`, Subscription: `65%` |
| [#5](../../issues/5) | Launch at login | macOS LaunchAgent |
| [#6](../../issues/6) | UI account type adaptation | Dollar costs vs quota percentage |

**Delivery criteria**: Glance at the menu bar and know today's spend.

---

## MVP 2: AI-Powered Analytics

> Goal: Evolve from "showing data" to "proactive alerts"

| Issue | Feature | Description |
|-------|---------|-------------|
| [#7](../../issues/7) | Historical trend charts | Daily spend/usage line chart |
| [#8](../../issues/8) | AI anomaly detection | Identify spending spikes from real data |
| [#9](../../issues/9) | Budget/quota prediction | Forecast exhaustion date |
| [#10](../../issues/10) | Smart notifications | Trend-based alerts, not just thresholds |

**Delivery criteria**: After a week of use, the app proactively tells you "today's spending is unusually high."

---

## MVP 3: Multi-Provider Expansion

> Goal: Deliver on the "unified monitoring" promise

| Issue | Feature | Description |
|-------|---------|-------------|
| [#11](../../issues/11) | OpenAI / Codex polish | Usage API integration + per-model breakdown |
| [#12](../../issues/12) | Cursor support | Local data reading |
| [#13](../../issues/13) | GitHub Copilot support | Org API or manual entry |
| [#14](../../issues/14) | Cross-provider comparison | Multi-tool cost comparison + efficiency analysis |

**Delivery criteria**: See Claude + OpenAI costs side-by-side in one panel.

---

## MVP 4: Release Ready

> Goal: Out-of-box experience, ready for public distribution

| Issue | Feature | Description |
|-------|---------|-------------|
| [#15](../../issues/15) | UI polish | Native feel, animations, light/dark theme |
| [#16](../../issues/16) | Official icons | App icon + tray icon design |
| [#17](../../issues/17) | Homebrew / dmg distribution | `brew install --cask aimeter` |
| [#18](../../issues/18) | Auto-update | tauri-plugin-updater |
| [#19](../../issues/19) | Onboarding wizard | First-run setup guide |

**Delivery criteria**: A first-time user can install and start using it without reading docs.

---

## Project Management

- **Milestones**: Each MVP maps to a [GitHub Milestone](../../milestones)
- **Issues**: Each feature is tracked as an Issue under its Milestone
- **PR workflow**: Branch per Issue → PR → AI auto-review → Merge
- **Progress**: [GitHub Milestones page](../../milestones)
