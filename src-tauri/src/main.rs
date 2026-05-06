#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod history;
mod notifications;
mod providers;
mod usage;

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl,
};
use tauri_plugin_autostart::MacosLauncher;

const PANEL_WIDTH: f64 = 360.0;
const PANEL_HEIGHT: f64 = 520.0;
const PANEL_LABEL: &str = "panel";

fn toggle_panel(app: &tauri::AppHandle, position: Option<tauri::PhysicalPosition<f64>>) {
    if let Some(window) = app.get_webview_window(PANEL_LABEL) {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
        return;
    }

    let mut builder = tauri::webview::WebviewWindowBuilder::new(
        app,
        PANEL_LABEL,
        WebviewUrl::App("index.html".into()),
    )
    .title("AIMeter")
    .inner_size(PANEL_WIDTH, PANEL_HEIGHT)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .focused(true)
    .visible(true);

    if let Some(pos) = position {
        let scale = app
            .primary_monitor()
            .ok()
            .flatten()
            .map(|m| m.scale_factor())
            .unwrap_or(1.0);
        let x = (pos.x / scale) - PANEL_WIDTH / 2.0;
        let y = pos.y / scale;
        builder = builder.position(x, y);
    }

    if let Ok(window) = builder.build() {
        let w = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                let _ = w.hide();
            }
        });
    }
}

fn format_tray_title(usages: &[usage::ProviderUsage]) -> String {
    if usages.is_empty() {
        return String::new();
    }
    let mut parts = Vec::new();
    for u in usages {
        if u.error.is_some() {
            continue;
        }
        match u.account_type {
            providers::AccountType::Api => {
                parts.push(format!("${:.2}", u.cost_used));
            }
            _ => {
                if let (Some(used), Some(limit)) = (u.quota_used, u.quota_limit) {
                    if limit > 0.0 {
                        parts.push(format!("{:.0}%", (used / limit) * 100.0));
                    }
                }
            }
        }
    }
    if parts.is_empty() {
        return String::new();
    }
    parts.join(" | ")
}

#[tauri::command]
async fn get_usage_summary(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<notifications::NotificationState>>,
) -> Result<Vec<usage::ProviderUsage>, String> {
    let usages = usage::get_all_usage(&app).await.map_err(|e| e.to_string())?;
    history::save_snapshot(&usages);
    if let Ok(mut notif_state) = state.lock() {
        notif_state.check_and_notify(&app, &usages);
    }
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title(Some(&format_tray_title(&usages)));
    }
    Ok(usages)
}

#[tauri::command]
async fn get_history(days: Option<usize>) -> Result<Vec<history::DailySnapshot>, String> {
    Ok(history::load_history(days.unwrap_or(90)))
}

#[tauri::command]
async fn get_providers(
    app: tauri::AppHandle,
) -> Result<Vec<providers::ProviderConfig>, String> {
    providers::load_providers(&app).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_providers(
    app: tauri::AppHandle,
    providers: Vec<providers::ProviderConfig>,
) -> Result<(), String> {
    providers::save_providers(&app, providers)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_insights(
    history: Vec<ai::insights::UsageDataPoint>,
    budget_limit: Option<f64>,
) -> Result<Vec<ai::insights::Insight>, String> {
    Ok(ai::insights::analyze_spending_patterns(&history, budget_limit))
}

#[tauri::command]
async fn get_predictions(
    history: Vec<ai::insights::UsageDataPoint>,
    budget_limit: Option<f64>,
    forecast_days: Option<usize>,
) -> Result<ai::predictions::Prediction, String> {
    Ok(ai::predictions::forecast_usage(
        &history,
        budget_limit,
        forecast_days.unwrap_or(14),
    ))
}

#[tauri::command]
async fn get_autostart_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;
    app.autolaunch()
        .is_enabled()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_autostart_enabled(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let autostart = app.autolaunch();
    if enabled {
        autostart.enable().map_err(|e| e.to_string())
    } else {
        autostart.disable().map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn compare_providers(
    provider_data: Vec<(String, Vec<ai::insights::UsageDataPoint>)>,
) -> Result<Vec<ai::insights::Insight>, String> {
    Ok(ai::insights::compare_providers(&provider_data))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .manage(Mutex::new(notifications::NotificationState::new()))
        .setup(|app| {
            // Initial data fetch + tray title update on startup
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(usages) = usage::get_all_usage(&handle).await {
                    history::save_snapshot(&usages);
                    if let Some(tray) = handle.tray_by_id("main") {
                        let _ = tray.set_title(Some(&format_tray_title(&usages)));
                    }
                }

                // Background refresh every 60 seconds
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                    if let Ok(usages) = usage::get_all_usage(&handle).await {
                        history::save_snapshot(&usages);
                        if let Some(tray) = handle.tray_by_id("main") {
                            let _ = tray.set_title(Some(&format_tray_title(&usages)));
                        }
                    }
                }
            });

            let quit = MenuItem::with_id(app, "quit", "Quit AIMeter", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            TrayIconBuilder::with_id("main")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        rect,
                        ..
                    } = event
                    {
                        let (px, py) = match rect.position {
                            tauri::Position::Physical(p) => (p.x as f64, p.y as f64),
                            tauri::Position::Logical(p) => (p.x, p.y),
                        };
                        let (sw, sh) = match rect.size {
                            tauri::Size::Physical(s) => (s.width as f64, s.height as f64),
                            tauri::Size::Logical(s) => (s.width, s.height),
                        };
                        let pos = tauri::PhysicalPosition {
                            x: px + sw / 2.0,
                            y: py + sh,
                        };
                        toggle_panel(tray.app_handle(), Some(pos));
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_usage_summary,
            get_history,
            get_providers,
            save_providers,
            get_insights,
            get_predictions,
            compare_providers,
            get_autostart_enabled,
            set_autostart_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running AIMeter");
}
