// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;
mod apis;

use apis::{
    get_database_info, 
    create_database_files, 
    create_stock, 
    get_all_stocks, 
    get_stock_by_symbol, 
    update_stock, 
    delete_stock, 
    add_stock_history, 
    get_stock_history
};
use database::{get_migrations, get_market_analysis_migrations, get_user_settings_migrations, initialize_databases, force_create_all_databases};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_db_path(db_name: &str) -> String {
    // 获取项目根目录路径
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let project_root = current_dir
        .parent() // 从 src-tauri 目录回到项目根目录
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("db")
        .join(db_name);
    
    format!("sqlite:{}", project_root.to_string_lossy())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 配置单个 SQL 插件实例，管理多个数据库
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(&get_db_path("stock_data.db"), get_migrations())
                .add_migrations(&get_db_path("market_analysis.db"), get_market_analysis_migrations())
                .add_migrations(&get_db_path("user_settings.db"), get_user_settings_migrations())
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            get_database_info,
            create_database_files,
            create_stock,
            get_all_stocks,
            get_stock_by_symbol,
            update_stock,
            delete_stock,
            add_stock_history,
            get_stock_history
        ])
        .setup(|app| {
            // 应用启动时初始化数据库连接
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // 确保项目根目录的 db 目录存在
                let current_dir = std::env::current_dir()
                    .unwrap_or_else(|_| std::path::PathBuf::from("."));
                let project_root = current_dir
                    .parent() // 从 src-tauri 目录回到项目根目录
                    .unwrap_or_else(|| std::path::Path::new("."));
                
                let db_dir = project_root.join("db");
                if !db_dir.exists() {
                    if let Err(e) = std::fs::create_dir_all(&db_dir) {
                        println!("创建 db 目录失败: {}", e);
                    } else {
                        println!("已创建 db 目录: {}", db_dir.display());
                    }
                }
                
                match initialize_databases(&app_handle).await {
                    Ok(result) => {
                        println!("数据库初始化成功: {}", result);
                        
                        // 强制创建所有数据库文件
                        match force_create_all_databases(&app_handle).await {
                            Ok(create_result) => {
                                println!("数据库文件创建结果: {}", create_result);
                            }
                            Err(e) => {
                                println!("数据库文件创建失败: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("数据库初始化失败: {}", e);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
