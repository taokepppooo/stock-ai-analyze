use crate::database::{
    get_database_status, CreateStockRequest, Stock, StockHistory, UpdateStockRequest,
    initialize_all_databases,
};
use std::collections::HashMap;
use tauri::{AppHandle, command};

#[command]
pub async fn create_stock(
    _app: AppHandle,
    stock_data: CreateStockRequest,
) -> Result<Stock, String> {
    println!("Creating stock: {:?}", stock_data);

    let stock = Stock {
        id: Some(1),
        symbol: stock_data.symbol,
        name: stock_data.name,
        price: stock_data.price,
        change_percent: stock_data.change_percent,
        volume: stock_data.volume,
        market_cap: stock_data.market_cap,
        sector: stock_data.sector,
        created_at: Some("2025-09-12 12:00:00".to_string()),
        updated_at: Some("2025-09-12 12:00:00".to_string()),
    };

    println!("Stock created successfully: {}", stock.symbol);
    Ok(stock)
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
pub async fn get_database_info(app: AppHandle) -> Result<HashMap<String, String>, String> {
    println!("Getting database status information...");
    match get_database_status(&app).await {
        Ok(status_map) => {
            println!("Database status retrieved: {:?}", status_map);
            Ok(status_map)
        }
        Err(e) => {
            println!("Failed to get database status: {}", e);
            Err(format!("Failed to get database status: {}", e))
        }
    }
}

#[command]
pub async fn create_database_files(_app: AppHandle) -> Result<String, String> {
    println!("Creating database files if missing...");

    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let project_root = current_dir
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let db_folder_path = project_root.join("db");

    if !db_folder_path.exists() {
        std::fs::create_dir_all(&db_folder_path)
            .map_err(|e| format!("Failed to create db folder: {}", e))?;
    }

    let db_files = vec![
        ("stock_data.db", "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY)"),
        ("market_analysis.db", "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY)"),
        ("user_settings.db", "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY)"),
    ];

    let mut results = Vec::new();

    for (db_name, sql) in db_files {
        let db_path = db_folder_path.join(db_name);
        match rusqlite::Connection::open(&db_path) {
            Ok(conn) => match conn.execute(sql, []) {
                Ok(_) => {
                    println!("✔ Database file ready: {}", db_path.display());
                    results.push(format!("{}: created/verified at {}", db_name, db_path.display()));
                }
                Err(e) => {
                    println!("✖ Database init failed ({}): {}", db_name, e);
                    results.push(format!("{}: init failed - {}", db_name, e));
                }
            },
            Err(e) => {
                println!("✖ Database open failed ({}): {}", db_name, e);
                results.push(format!("{}: open failed - {}", db_name, e));
            }
        }
    }

    Ok(format!("Database file creation results:\n{}", results.join("\n")))
}

#[command]
pub async fn initialize_databases_with_tables(app: AppHandle) -> Result<String, String> {
    println!("开始初始化数据库并创建表...");
    
    match initialize_all_databases(&app).await {
        Ok(result) => {
            println!("数据库初始化完成: {}", result);
            Ok(result)
        }
        Err(e) => {
            println!("数据库初始化失败: {}", e);
            Err(format!("数据库初始化失败: {}", e))
        }
    }
}