# rust-anyhow-serde-chrono-csv

Rust 基础工程化工具链示例：anyhow、serde、chrono、csv

## 项目说明

| 依赖库   | 用途 |
|---------|------|
| **anyhow** | 应用层错误处理：`?` 传播、Context、`Result<T, Error>` |
| **serde**  | 通用序列化/反序列化（JSON），用于配置、API 响应等 |
| **chrono** | 日期时间处理：K 线时间戳、回测区间、时区 |
| **csv**    | CSV 读写，行情/因子数据导入导出 |

## 构建与运行

```bash
cargo build
cargo run
```

## 运行效果

程序会依次演示：

- **anyhow** — 解析、`?` 传播、错误上下文
- **serde**  — JSON 序列化与反序列化（如 `Trade`、`Config` 等结构体）
- **chrono** — UTC/本地时间、指定日期时间、时长、Unix 时间戳
- **csv**    — 写入并读回 CSV（如 OHLCV K 线数据）

## 许可证

MIT 或 Apache-2.0，任选其一。
