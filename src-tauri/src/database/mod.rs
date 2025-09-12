pub mod init;
pub mod models;

pub use models::*;
pub use init::*;

use tauri::AppHandle;
use std::collections::HashMap;

pub async fn get_database_status(_app: &AppHandle) -> Result<HashMap<String, String>, String> {
    println!("正在检查数据库状态...");

    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let project_root = current_dir
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let db_folder_path = project_root.join("db");

    let db_files = vec!["stock_data.db", "market_analysis.db", "user_settings.db"];
    let mut status_map = HashMap::new();

    for db_name in db_files {
        let db_path = db_folder_path.join(db_name);
        let status = if db_path.exists() {
            format!("存在 (路径: {})", db_path.display())
        } else {
            format!("不存在 (期望路径: {})", db_path.display())
        };
        status_map.insert(db_name.to_string(), status);
    }

    println!("数据库状态检查完成: {:?}", status_map);
    Ok(status_map)
}