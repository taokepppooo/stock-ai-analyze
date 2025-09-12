mod apis;
mod database;

use apis::*;
use database::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default().build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            create_stock,
            get_all_stocks,
            get_stock_by_symbol,
            update_stock,
            delete_stock,
            add_stock_history,
            get_stock_history,
            get_database_info,
            create_database_files,
            initialize_databases_with_tables
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match initialize_all_databases(&app_handle).await {
                    Ok(msg) => println!("数据库初始化成功: {}", msg),
                    Err(e) => println!("数据库初始化失败: {}", e),
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
