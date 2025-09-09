use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    pub id: Option<i64>,
    pub symbol: String,
    pub name: String,
    pub price: Option<f64>,
    pub change_percent: Option<f64>,
    pub volume: Option<i64>,
    pub market_cap: Option<f64>,
    pub sector: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStockRequest {
    pub symbol: String,
    pub name: String,
    pub price: Option<f64>,
    pub change_percent: Option<f64>,
    pub volume: Option<i64>,
    pub market_cap: Option<f64>,
    pub sector: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStockRequest {
    pub price: Option<f64>,
    pub change_percent: Option<f64>,
    pub volume: Option<i64>,
    pub market_cap: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockHistory {
    pub id: Option<i64>,
    pub symbol: String,
    pub price: f64,
    pub volume: Option<i64>,
    pub timestamp: Option<String>,
}
