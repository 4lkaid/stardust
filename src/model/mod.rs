pub mod account;
pub mod account_log;
pub mod action_type;
pub mod asset_type;

use axum_kit::postgres;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Serializer;

fn serialize_utc_to_session_tz<S>(
    utc_time: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 直接复用现有会话时区获取逻辑
    let tz: Tz = postgres::pg_session_timezone()
        .parse()
        .map_err(|_| serde::ser::Error::custom("解析时区失败"))?;

    // 转换为会话时区并序列化
    let local_time = utc_time.with_timezone(&tz);
    serializer.serialize_str(&local_time.to_rfc3339())
}
