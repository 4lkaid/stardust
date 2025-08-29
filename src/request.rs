use crate::{
    constant::{MAX_PAGE_SIZE, MIN_PAGE, MIN_PAGE_SIZE},
    service::{action_type::ActionTypeService, asset_type::AssetTypeService},
};
use serde::{Deserialize, Deserializer};
use sqlx::types::Decimal;
use std::{borrow::Cow, str::FromStr as _};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate, Debug)]
pub struct AccountRequest {
    #[validate(range(min = 1, message = "用户ID必须为正整数"))]
    pub user_id: i32,
    #[validate(custom(function = "validate_asset_type_id"))]
    pub asset_type_id: i32,
}

#[derive(Deserialize, Validate, Debug)]
pub struct AccountsRequest {
    #[validate(range(min = 1, message = "用户ID必须为正整数"))]
    pub user_id: i32,
}

#[derive(Deserialize, Validate, Debug)]
pub struct AccountActionRequest {
    #[validate(range(min = 1, message = "用户ID必须为正整数"))]
    pub user_id: i32,
    #[validate(custom(function = "validate_asset_type_id"))]
    pub asset_type_id: i32,
    #[validate(custom(function = "validate_action_type_id"))]
    pub action_type_id: i32,
    #[validate(custom(function = "validate_amount"))]
    #[serde(deserialize_with = "deserialize_decimal")]
    pub amount: Decimal,
    #[validate(length(min = 32, message = "订单号长度至少32位"))]
    pub order_number: String,
    #[validate(length(min = 1, message = "描述不能为空"))]
    pub description: String,
}

#[derive(Deserialize, Validate, Debug)]
#[validate(schema(function = "validate_time_range"))]
pub struct AccountLogRequest {
    #[validate(range(min = 1, message = "用户ID必须为正整数"))]
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
        if let (Ok(start), Ok(end)) = (start, end)
            && start > end
        {
            return Err(ValidationError::new("time_range")
                .with_message(Cow::Borrowed("结束时间不能早于开始时间")));
        }
    }
    Ok(())
}

fn validate_asset_type_id(id: i32) -> Result<(), ValidationError> {
    if !AssetTypeService::is_active(id) {
        return Err(
            ValidationError::new("asset_type_id").with_message(Cow::Borrowed("无效的资产类型"))
        );
    }
    Ok(())
}

fn validate_action_type_id(id: i32) -> Result<(), ValidationError> {
    if !ActionTypeService::is_active(id) {
        return Err(
            ValidationError::new("action_type_id").with_message(Cow::Borrowed("无效的操作类型"))
        );
    }
    Ok(())
}

fn validate_amount(amount: &Decimal) -> Result<(), ValidationError> {
    if amount.scale() > 6 {
        return Err(
            ValidationError::new("amount").with_message(Cow::Borrowed("金额最多支持6位小数"))
        );
    }
    let min_amount = Decimal::from_str("0.000001").unwrap();
    if amount < &min_amount {
        return Err(
            ValidationError::new("amount").with_message(Cow::Borrowed("金额不能小于0.000001"))
        );
    }
    Ok(())
}

fn validate_date_format(date: &str) -> Result<(), ValidationError> {
    if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
        return Err(ValidationError::new("date_format")
            .with_message(Cow::Borrowed("日期格式应为YYYY-MM-DD")));
    }
    Ok(())
}

fn deserialize_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Decimal::from_str(&s).map_err(|_| serde::de::Error::custom("请输入有效金额"))
}
