#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod providers;
mod usage;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

#[tauri::command]
async fn get_usage_summary() -> Result<Vec<usage::ProviderUsage>, String> {
    usage::get_all_usage().await.map_err(|e| e.to_string())
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
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit AIMeter", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    _ => {}
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
