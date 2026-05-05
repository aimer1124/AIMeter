#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod notifications;
mod providers;
mod usage;

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl,
};

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

#[tauri::command]
async fn get_usage_summary(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<notifications::NotificationState>>,
) -> Result<Vec<usage::ProviderUsage>, String> {
    let usages = usage::get_all_usage(&app).await.map_err(|e| e.to_string())?;
    if let Ok(mut notif_state) = state.lock() {
        notif_state.check_and_notify(&app, &usages);
    }
    Ok(usages)
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
async fn compare_providers(
    provider_data: Vec<(String, Vec<ai::insights::UsageDataPoint>)>,
) -> Result<Vec<ai::insights::Insight>, String> {
    Ok(ai::insights::compare_providers(&provider_data))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .manage(Mutex::new(notifications::NotificationState::new()))
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit AIMeter", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            TrayIconBuilder::new()
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
            get_providers,
            save_providers,
            get_insights,
            get_predictions,
            compare_providers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running AIMeter");
}
