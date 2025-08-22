use anyhow::{Result, anyhow};
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;

/// 起止时间边界
pub enum DayBoundary {
    Start,
    End,
}

/// 把日期字符串解析为当天的 UTC 边界时间
pub fn parse_day_boundary(date_str: &str, tz: Tz, boundary: DayBoundary) -> Result<DateTime<Utc>> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

    let dt = match boundary {
        DayBoundary::Start => tz
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
            .single()
            .ok_or_else(|| anyhow!("日期{}在{}时区无效", date_str, tz))?,
        DayBoundary::End => {
            let next_day = date
                .succ_opt()
                .ok_or_else(|| anyhow!("日期{}无下一天", date_str))?;
            tz.with_ymd_and_hms(next_day.year(), next_day.month(), next_day.day(), 0, 0, 0)
                .earliest()
                .ok_or_else(|| anyhow!("日期{}的下一天在{}时区无效", date_str, tz))?
                - Duration::microseconds(1)
        }
    };

    Ok(dt.with_timezone(&Utc))
}
