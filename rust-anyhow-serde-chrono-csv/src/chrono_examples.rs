//! chrono：日期时间处理（K 线时间戳、回测区间、时区）

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, Utc};

/// 常用日期时间操作
pub fn run() {
    let now_utc: DateTime<Utc> = Utc::now();
    let now_local: DateTime<Local> = Local::now();
    println!("[chrono] UTC 当前: {}", now_utc.format("%Y-%m-%d %H:%M:%S"));
    println!("[chrono] 本地当前: {}", now_local.format("%Y-%m-%d %H:%M:%S"));

    let naive = NaiveDate::from_ymd_opt(2024, 3, 15)
        .unwrap()
        .and_hms_opt(9, 30, 0)
        .unwrap();
    println!("[chrono] 指定时间: {}", naive);

    let later = naive + Duration::hours(2);
    println!("[chrono] 2 小时后: {}", later);

    let ts = now_utc.timestamp();
    println!("[chrono] Unix 时间戳: {}", ts);

    let from_ts = DateTime::from_timestamp(ts, 0).unwrap();
    println!("[chrono] 时间戳转回: {}", from_ts);
}
