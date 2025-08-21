use crate::{
    model::{account::AccountModel, account_log::AccountLogModel},
    request::{AccountActionRequest, AccountLogRequest, AccountRequest, AccountsRequest},
    service::account::AccountService,
};
use axum::{Json, http::StatusCode};
use axum_kit::{AppResult, validation::ValidatedJson};

// 添加账户
pub async fn create(
    ValidatedJson(payload): ValidatedJson<AccountRequest>,
) -> AppResult<(StatusCode, Json<AccountModel>)> {
    let account = AccountService::create(&payload).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

// 账户信息
pub async fn info(
    ValidatedJson(payload): ValidatedJson<AccountRequest>,
) -> AppResult<Json<AccountModel>> {
    let account = AccountService::info(&payload).await?;
    Ok(Json(account))
}

// 某`user_id`所有账户信息
pub async fn infos(
    ValidatedJson(payload): ValidatedJson<AccountsRequest>,
) -> AppResult<Json<Vec<AccountModel>>> {
    let accounts = AccountService::infos(&payload).await?;
    Ok(Json(accounts))
}

// 账户操作
// 仅涉及可用余额、冻结余额、累计收入、累计支出的变更
pub async fn actions(
    ValidatedJson(payload): ValidatedJson<Vec<AccountActionRequest>>,
) -> AppResult<()> {
    AccountService::actions(&payload).await
}

// 账户操作记录
pub async fn logs(
    ValidatedJson(payload): ValidatedJson<AccountLogRequest>,
) -> AppResult<Json<Vec<AccountLogModel>>> {
    let account_logs = AccountService::logs(&payload).await?;
    Ok(Json(account_logs))
}
