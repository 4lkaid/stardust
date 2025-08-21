use crate::{
    constant::{MAX_PAGE_SIZE, MIN_PAGE, MIN_PAGE_SIZE},
    service::{action_type::ActionTypeService, asset_type::AssetTypeService},
};
use serde::{Deserialize, Deserializer};
use sqlx::types::Decimal;
use std::str::FromStr as _;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate, Debug)]
pub struct AccountRequest {
    #[validate(range(min = 1))]
    pub user_id: i32,
    #[validate(custom(function = "validate_asset_type_id"))]
    pub asset_type_id: i32,
}

#[derive(Deserialize, Validate, Debug)]
pub struct AccountsRequest {
    #[validate(range(min = 1))]
    pub user_id: i32,
}

#[derive(Deserialize, Validate, Debug)]
pub struct AccountActionRequest {
    #[validate(range(min = 1))]
    pub user_id: i32,
    #[validate(custom(function = "validate_asset_type_id"))]
    pub asset_type_id: i32,
    #[validate(custom(function = "validate_action_type_id"))]
    pub action_type_id: i32,
    #[validate(custom(function = "validate_amount"))]
    #[serde(deserialize_with = "deserialize_decimal")]
    pub amount: Decimal,
    #[validate(length(min = 32))]
    pub order_number: String,
    #[validate(length(min = 1))]
    pub description: String,
}

#[derive(Deserialize, Validate, Debug)]
#[validate(schema(function = "validate_time_range"))]
pub struct AccountLogRequest {
    #[validate(range(min = 1))]
    pub user_id: i32,
    #[validate(custom(function = "validate_asset_type_id"))]
    pub asset_type_id: i32,
    #[validate(custom(function = "validate_action_type_id"))]
    pub action_type_id: Option<i32>,
    #[validate(custom(function = "validate_date_format"))]
    pub start_time: Option<String>,
    #[validate(custom(function = "validate_date_format"))]
    pub end_time: Option<String>,
    #[validate(range(min = "MIN_PAGE"))]
    pub page: i32,
    #[validate(range(min = "MIN_PAGE_SIZE", max = "MAX_PAGE_SIZE"))]
    pub page_size: i32,
}

fn validate_time_range(request: &AccountLogRequest) -> Result<(), ValidationError> {
    if let (Some(start_time), Some(end_time)) = (&request.start_time, &request.end_time) {
        let start = chrono::NaiveDate::parse_from_str(start_time, "%Y-%m-%d");
        let end = chrono::NaiveDate::parse_from_str(end_time, "%Y-%m-%d");
        if let (Ok(start), Ok(end)) = (start, end) {
            if start > end {
                return Err(ValidationError::new("end_time 必须大于等于 start_time"));
            }
        }
    }
    Ok(())
}

fn validate_asset_type_id(id: i32) -> Result<(), ValidationError> {
    if !AssetTypeService::is_active(id) {
        return Err(ValidationError::new("无效值"));
    }
    Ok(())
}

fn validate_action_type_id(id: i32) -> Result<(), ValidationError> {
    if !ActionTypeService::is_active(id) {
        return Err(ValidationError::new("无效值"));
    }
    Ok(())
}

fn validate_amount(amount: &Decimal) -> Result<(), ValidationError> {
    if amount.scale() > 6 {
        return Err(ValidationError::new(
            "金额小数位最多为6位（例如最多保留\"1.234567\"）",
        ));
    }
    let min_amount = Decimal::from_str("0.000001").unwrap();
    if amount < &min_amount {
        return Err(ValidationError::new("金额不能小于0.000001（即1e-6）"));
    }
    Ok(())
}

fn validate_date_format(date: &str) -> Result<(), ValidationError> {
    if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
        return Err(ValidationError::new("无效值"));
    }
    Ok(())
}

fn deserialize_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Decimal::from_str(&s).map_err(|_| serde::de::Error::custom("金额格式错误，请输入有效的数字字符串（支持整数或最多6位小数的小数，例如\"100\"、\"1.234567\"）"))
}
