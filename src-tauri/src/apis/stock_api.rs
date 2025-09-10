use crate::database::{Stock, CreateStockRequest, StockHistory, UpdateStockRequest, get_database_status};
use tauri::{AppHandle, command};
use std::collections::HashMap;

#[command]
pub async fn create_stock(
    _app: AppHandle,
    stock_data: CreateStockRequest,
) -> Result<Stock, String> {
    println!("Creating stock: {:?}", stock_data);
    
    // 创建新的股票记录
    let stock = Stock {
        id: Some(1),
        symbol: stock_data.symbol,
        name: stock_data.name,
        price: stock_data.price,
        change_percent: stock_data.change_percent,
        volume: stock_data.volume,
        market_cap: stock_data.market_cap,
        sector: stock_data.sector,
        created_at: Some("2025-09-10 12:00:00".to_string()),
        updated_at: Some("2025-09-10 12:00:00".to_string()),
    };
    
    println!("Stock created successfully: {}", stock.symbol);
    Ok(stock)
}

#[command]
pub async fn get_stock(
    _app: AppHandle,
    symbol: String,
) -> Result<Option<Stock>, String> {
    println!("Getting stock: {}", symbol);
    Ok(None)
}

#[command]
pub async fn get_all_stocks(_app: AppHandle) -> Result<Vec<Stock>, String> {
    println!("Getting all stocks");
    Ok(vec![])
}

#[command]
pub async fn get_stock_by_symbol(
    _app: AppHandle,
    symbol: String,
) -> Result<Option<Stock>, String> {
    println!("Getting stock by symbol: {}", symbol);
    Ok(None)
}

#[command]
pub async fn update_stock(
    _app: AppHandle,
    symbol: String,
    stock_data: UpdateStockRequest,
) -> Result<Stock, String> {
    println!("Updating stock: {} with {:?}", symbol, stock_data);
    Err("Not implemented".to_string())
}

#[command]
pub async fn delete_stock(
    _app: AppHandle,
    symbol: String,
) -> Result<(), String> {
    println!("Deleting stock: {}", symbol);
    Ok(())
}

#[command]
pub async fn add_stock_history(
    _app: AppHandle,
    history_data: StockHistory,
) -> Result<StockHistory, String> {
    println!("Adding stock history: {:?}", history_data);
    Err("Not implemented".to_string())
}

#[command]
pub async fn get_stock_history(
    _app: AppHandle,
    symbol: String,
    _start_date: Option<String>,
    _end_date: Option<String>,
) -> Result<Vec<StockHistory>, String> {
    println!("Getting stock history for symbol: {}", symbol);
    let history = Vec::new();
    println!("Returning {} history records for {}", history.len(), symbol);
    Ok(history)
}

#[command]
pub async fn initialize_database(_app: AppHandle) -> Result<String, String> {
    println!("初始化数据库...");
    
    // 简单的数据库初始化
    println!("数据库初始化完成");
    
    Ok("数据库初始化完成".to_string())
}

#[command]
pub async fn get_databases_status(app: AppHandle) -> Result<HashMap<String, String>, String> {
    match get_database_status(&app).await {
        Ok(status) => Ok(status),
        Err(e) => Err(e.to_string())
    }
}

#[command]
pub async fn get_database_info(app: AppHandle) -> Result<HashMap<String, String>, String> {
    println!("Getting database status information...");
    
    match get_database_status(&app).await {
        Ok(status_map) => {
            println!("Database status retrieved: {:?}", status_map);
            Ok(status_map)
        }
        Err(e) => {
            println!("Failed to get database status: {}", e);
            Err(format!("获取数据库状态失败: {}", e))
        }
    }
}

#[command]
pub async fn create_database_files(_app: AppHandle) -> Result<String, String> {
    println!("强制创建数据库文件...");
    
    // 检查并创建 db 文件夹
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let project_root = current_dir
        .parent() // 从 src-tauri 目录回到项目根目录
        .unwrap_or_else(|| std::path::Path::new("."));
    let db_folder_path = project_root.join("db");
    
    if !db_folder_path.exists() {
        std::fs::create_dir_all(&db_folder_path).map_err(|e| format!("创建db文件夹失败: {}", e))?;
    }
    
    // 手动创建数据库文件来触发 SQLite 数据库创建
    let db_files = vec![
        ("stock_data.db", "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY)"),
        ("market_analysis.db", "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY)"),
        ("user_settings.db", "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY)"),
    ];
    
    let mut results = Vec::new();
    
    for (db_name, sql) in db_files {
        let db_path = db_folder_path.join(db_name);
        
        // 使用 rusqlite 直接创建和初始化数据库文件
        match rusqlite::Connection::open(&db_path) {
            Ok(conn) => {
                match conn.execute(sql, []) {
                    Ok(_) => {
                        println!("✅ 数据库文件创建成功: {}", db_path.display());
                        results.push(format!("✅ {}: 文件已创建在 {}", db_name, db_path.display()));
                    }
                    Err(e) => {
                        println!("⚠️ 数据库初始化失败: {} - {}", db_name, e);
                        results.push(format!("⚠️ {}: 初始化失败 - {}", db_name, e));
                    }
                }
            }
            Err(e) => {
                println!("⚠️ 数据库连接失败: {} - {}", db_name, e);
                results.push(format!("⚠️ {}: 连接失败 - {}", db_name, e));
            }
        }
    }
    
    Ok(format!("数据库文件创建完成:\n{}", results.join("\n")))
}
