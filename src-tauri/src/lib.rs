// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;
mod apis;

use apis::*;
use database::get_migrations;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:stock_data.db", get_migrations())
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            create_stock,
            get_all_stocks,
            get_stock_by_symbol,
            update_stock,
            delete_stock,
            add_stock_history,
            get_stock_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
