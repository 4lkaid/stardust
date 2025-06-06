use crate::{
    constant::{MAX_PAGE_SIZE, MIN_PAGE, MIN_PAGE_SIZE},
    service::{action_type::ActionTypeService, asset_type::AssetTypeService},
};
use axum::http::StatusCode;
use axum_kit::{AppResult, error::Error};
use num_traits::FromPrimitive;
use serde::Deserialize;
use sqlx::types::Decimal;
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
    #[validate(range(min = 0.000001), custom(function = "validate_amount"))]
    pub amount: f64,
    #[validate(length(min = 32))]
    pub order_number: String,
    #[validate(length(min = 1))]
    pub description: String,
}

#[derive(Deserialize, Validate, Debug)]
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

impl AccountLogRequest {
    pub fn validate_time_range(&self) -> AppResult<()> {
        if let (Some(start_time), Some(end_time)) = (&self.start_time, &self.end_time) {
            let start = chrono::NaiveDate::parse_from_str(start_time, "%Y-%m-%d");
            let end = chrono::NaiveDate::parse_from_str(end_time, "%Y-%m-%d");
            if let (Ok(start), Ok(end)) = (start, end) {
                if start > end {
                    return Err(Error::Custom(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "end_time 必须大于等于 start_time".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
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

fn validate_amount(amount: f64) -> Result<(), ValidationError> {
    if Decimal::from_f64(amount).unwrap().scale() > 6 {
        return Err(ValidationError::new("无效值(最多6位小数)"));
    }
    Ok(())
}

fn validate_date_format(date: &str) -> Result<(), ValidationError> {
    if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
        return Err(ValidationError::new("无效值"));
    }
    Ok(())
}
