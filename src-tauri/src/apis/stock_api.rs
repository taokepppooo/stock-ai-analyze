use crate::database::{Stock, CreateStockRequest, UpdateStockRequest, StockHistory};
use tauri::AppHandle;

// 创建股票记录
#[tauri::command]
pub async fn create_stock(
    app: AppHandle,
    stock_data: CreateStockRequest,
) -> Result<String, String> {
    println!("Creating stock: {:?}", stock_data);
    Ok(format!("Stock {} created successfully", stock_data.symbol))
}

// 获取所有股票
#[tauri::command]
pub async fn get_all_stocks(app: AppHandle) -> Result<Vec<Stock>, String> {
    // 返回示例数据
    let stocks = vec![
        Stock {
            id: Some(1),
            symbol: "AAPL".to_string(),
            name: "Apple Inc.".to_string(),
            price: Some(150.0),
            change_percent: Some(2.5),
            volume: Some(1000000),
            market_cap: Some(2500000000.0),
            sector: Some("Technology".to_string()),
            created_at: Some("2025-09-09 10:00:00".to_string()),
            updated_at: Some("2025-09-09 10:00:00".to_string()),
        },
        Stock {
            id: Some(2),
            symbol: "GOOGL".to_string(),
            name: "Alphabet Inc.".to_string(),
            price: Some(2800.0),
            change_percent: Some(-1.2),
            volume: Some(500000),
            market_cap: Some(1800000000.0),
            sector: Some("Technology".to_string()),
            created_at: Some("2025-09-09 10:00:00".to_string()),
            updated_at: Some("2025-09-09 10:00:00".to_string()),
        },
    ];
    
    Ok(stocks)
}

// 根据股票代码获取股票
#[tauri::command]
pub async fn get_stock_by_symbol(
    app: AppHandle,
    symbol: String,
) -> Result<Option<Stock>, String> {
    println!("Getting stock by symbol: {}", symbol);
    
    if symbol == "AAPL" {
        Ok(Some(Stock {
            id: Some(1),
            symbol: "AAPL".to_string(),
            name: "Apple Inc.".to_string(),
            price: Some(150.0),
            change_percent: Some(2.5),
            volume: Some(1000000),
            market_cap: Some(2500000000.0),
            sector: Some("Technology".to_string()),
            created_at: Some("2025-09-09 10:00:00".to_string()),
            updated_at: Some("2025-09-09 10:00:00".to_string()),
        }))
    } else {
        Ok(None)
    }
}

// 更新股票信息
#[tauri::command]
pub async fn update_stock(
    app: AppHandle,
    symbol: String,
    update_data: UpdateStockRequest,
) -> Result<Stock, String> {
    println!("Updating stock: {} with data: {:?}", symbol, update_data);
    
    // 返回更新后的示例数据
    Ok(Stock {
        id: Some(1),
        symbol,
        name: "Updated Stock".to_string(),
        price: update_data.price,
        change_percent: update_data.change_percent,
        volume: update_data.volume,
        market_cap: update_data.market_cap,
        sector: Some("Technology".to_string()),
        created_at: Some("2025-09-09 10:00:00".to_string()),
        updated_at: Some("2025-09-09 12:00:00".to_string()),
    })
}

// 删除股票
#[tauri::command]
pub async fn delete_stock(app: AppHandle, symbol: String) -> Result<bool, String> {
    println!("Deleting stock: {}", symbol);
    Ok(true)
}

// 添加股票历史记录
#[tauri::command]
pub async fn add_stock_history(
    app: AppHandle,
    symbol: String,
    price: f64,
    volume: Option<i64>,
) -> Result<StockHistory, String> {
    println!("Adding stock history for {}: price={}, volume={:?}", symbol, price, volume);
    
    Ok(StockHistory {
        id: Some(1),
        symbol,
        price,
        volume,
        timestamp: Some("2025-09-09 12:00:00".to_string()),
    })
}

// 获取股票历史记录
#[tauri::command]
pub async fn get_stock_history(
    app: AppHandle,
    symbol: String,
    limit: Option<i64>,
) -> Result<Vec<StockHistory>, String> {
    println!("Getting stock history for {}, limit: {:?}", symbol, limit);
    
    let history = vec![
        StockHistory {
            id: Some(1),
            symbol: symbol.clone(),
            price: 150.0,
            volume: Some(1000000),
            timestamp: Some("2025-09-09 09:00:00".to_string()),
        },
        StockHistory {
            id: Some(2),
            symbol: symbol.clone(),
            price: 148.5,
            volume: Some(950000),
            timestamp: Some("2025-09-09 10:00:00".to_string()),
        },
        StockHistory {
            id: Some(3),
            symbol: symbol.clone(),
            price: 152.0,
            volume: Some(1100000),
            timestamp: Some("2025-09-09 11:00:00".to_string()),
        },
    ];
    
    Ok(history)
}
