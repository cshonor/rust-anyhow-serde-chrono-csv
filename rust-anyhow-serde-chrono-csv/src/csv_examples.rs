//! csv：CSV 格式数据读写（行情/因子数据导入导出）

use anyhow::Result;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
struct Bar {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

/// 写 CSV 再读回（内存 Cursor 演示，实际可用 File）
pub fn run() -> Result<()> {
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

    wtr.write_record(&["date", "open", "high", "low", "close", "volume"])?;
    wtr.write_record(&["2024-03-01", "100", "105", "99", "103", "10000"])?;
    wtr.write_record(&["2024-03-02", "103", "108", "102", "106", "12000"])?;
    wtr.sync()?;

    let data = wtr.into_inner().map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let csv_str = String::from_utf8(data)?;
    println!("[csv] 写入的 CSV:\n{}", csv_str);

    let mut rdr = Reader::from_reader(csv_str.as_bytes());
    for (i, result) in rdr.deserialize().enumerate() {
        let bar: Bar = result?;
        println!("[csv] 第 {} 行: {:?}", i + 1, bar);
    }

    Ok(())
}
