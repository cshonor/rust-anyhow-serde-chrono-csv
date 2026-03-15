//! serde：通用序列化/反序列化（JSON 等，为量化配置、API 响应打基础）

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Trade {
    symbol: String,
    price: f64,
    qty: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
    timeout_secs: u64,
}

/// JSON 序列化与反序列化
pub fn run() {
    let trade = Trade {
        symbol: "BTCUSDT".to_string(),
        price: 43250.5,
        qty: 100,
    };

    let json = serde_json::to_string_pretty(&trade).unwrap();
    println!("[serde] 序列化 Trade:\n{}", json);

    let parsed: Trade = serde_json::from_str(&json).unwrap();
    println!("[serde] 反序列化: {:?}", parsed);

    let config = Config {
        api_key: "xxx".to_string(),
        timeout_secs: 30,
    };
    let config_json = serde_json::to_value(&config).unwrap();
    println!("[serde] Config as Value: {}", config_json);
}
