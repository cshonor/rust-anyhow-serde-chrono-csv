//! anyhow：简化应用层错误处理（? 传播、Context、Result<T, Error>）

use anyhow::{Context, Result};

/// 模拟可能失败的解析
fn parse_quantity(s: &str) -> Result<u32> {
    s.parse::<u32>().context("解析数量失败")
}

/// 模拟读取配置
fn load_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("读取配置失败: {}", path))
}

/// 链式 ? 与 context 添加上下文
pub fn run() -> Result<()> {
    let qty = parse_quantity("42")?;
    println!("[anyhow] 解析数量: {}", qty);

    let bad = parse_quantity("abc");
    if let Err(e) = bad {
        println!("[anyhow] 错误示例: {}", e);
        println!("[anyhow] 链式原因: {:?}", e.root_cause());
    }

    // 可选：尝试读不存在的文件，仅演示错误链
    let cfg = load_config("nonexistent.toml");
    if let Err(e) = cfg {
        println!("[anyhow] 带上下文的错误: {}", e);
    }

    Ok(())
}
