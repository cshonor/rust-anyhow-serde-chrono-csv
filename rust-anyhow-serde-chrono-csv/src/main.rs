//! rust-anyhow-serde-chrono-csv：基础工程化工具链示例

mod anyhow_examples;
mod chrono_examples;
mod csv_examples;
mod serde_examples;

fn main() -> anyhow::Result<()> {
    println!("=== anyhow 错误处理 ===\n");
    anyhow_examples::run()?;

    println!("\n=== serde 序列化/反序列化 ===\n");
    serde_examples::run();

    println!("\n=== chrono 日期时间 ===\n");
    chrono_examples::run();

    println!("\n=== csv 读写 ===\n");
    csv_examples::run()?;

    Ok(())
}
