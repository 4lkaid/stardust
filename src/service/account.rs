use super::{action_type::ActionTypeService, asset_type::AssetTypeService};
use crate::{
    model::{
        account::AccountModel,
        account_log::AccountLogModel,
        action_type::{ActionTypeModel, Change},
    },
    request::{AccountActionRequest, AccountLogRequest, AccountRequest, AccountsRequest},
    utils,
};
use axum::http::StatusCode;
use axum_kit::{AppResult, error::Error, postgres};
use sqlx::types::Decimal;
use validator::Validate;

pub struct AccountService;

impl AccountService {
    // #[allow(dead_code)]
    // pub async fn check_account_is_active(user_id: i32, asset_type_id: i32) -> AppResult<()> {
    //     if !AccountModel::is_active(postgres::conn(), user_id, asset_type_id).await {
    //         return Err(Error::Custom(
    //             StatusCode::FORBIDDEN,
    //             "操作失败，存在未启用账户".to_string(),
    //         ));
    //     }
    //     Ok(())
    // }

    pub async fn check_balance_before_update(
        action_type: &ActionTypeModel,
        account: &AccountModel,
        amount: Decimal,
    ) -> AppResult<()> {
        if (action_type.available_balance_change == Change::Dec
            && account.available_balance < amount)
            || (action_type.frozen_balance_change == Change::Dec && account.frozen_balance < amount)
        {
            return Err(Error::Custom(
                StatusCode::PAYMENT_REQUIRED,
                "操作失败，存在余额不足的账户".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn check_balance_after_update(
        action_type: &ActionTypeModel,
        account: &AccountModel,
    ) -> AppResult<()> {
        if (action_type.available_balance_change == Change::Dec
            && account.available_balance.is_sign_negative())
            || (action_type.frozen_balance_change == Change::Dec
                && account.frozen_balance.is_sign_negative())
        {
            return Err(Error::Custom(
                StatusCode::PAYMENT_REQUIRED,
                "操作失败，存在余额不足的账户".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn check_account_log_exists(
        account_id: i32,
        action_type_id: i32,
        order_number: &str,
    ) -> AppResult<()> {
        if AccountLogModel::is_exists(postgres::conn(), account_id, action_type_id, order_number)
            .await
        {
            return Err(Error::Custom(
                StatusCode::CONFLICT,
                "操作失败，存在已处理的订单".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn create(account_request: &AccountRequest) -> AppResult<AccountModel> {
        account_request.validate()?;
        let pool = postgres::conn();
        if AccountModel::is_exists(pool, account_request.user_id, account_request.asset_type_id)
            .await
        {
            return Err(Error::Custom(
                StatusCode::CONFLICT,
                "添加失败，该账户已存在".to_string(),
            ));
        }
        let account =
            AccountModel::create(pool, account_request.user_id, account_request.asset_type_id)
                .await?;
        Ok(account)
    }

    pub async fn info(account_request: &AccountRequest) -> AppResult<AccountModel> {
        account_request.validate()?;
        let account = AccountModel::find(
            postgres::conn(),
            account_request.user_id,
            account_request.asset_type_id,
        )
        .await?;
        Ok(account)
    }

    pub async fn infos(accounts_request: &AccountsRequest) -> AppResult<Vec<AccountModel>> {
        let accounts = AccountModel::find_multiple(
            postgres::conn(),
            accounts_request.user_id,
            AssetTypeService::ids(),
        )
        .await?;
        Ok(accounts)
    }

    pub async fn actions(account_action_requests: &Vec<AccountActionRequest>) -> AppResult<()> {
        account_action_requests.validate()?;
        let mut tx = postgres::conn().begin().await?;
        for account_action_request in account_action_requests {
            let account = AccountModel::find_for_update(
                &mut *tx,
                account_action_request.user_id,
                account_action_request.asset_type_id,
            )
            .await?;
            if !account.is_active {
                return Err(Error::Custom(
                    StatusCode::FORBIDDEN,
                    "操作失败，存在未启用账户".to_string(),
                ));
            }
            let action_type =
                ActionTypeService::by_id(account_action_request.action_type_id).unwrap();
            Self::check_balance_before_update(
                action_type,
                &account,
                account_action_request.amount.abs().trunc_with_scale(6),
            )
            .await?;
            Self::check_account_log_exists(
                account.id,
                action_type.id,
                account_action_request.order_number.as_str(),
            )
            .await?;
            Self::update_balance(&mut tx, account_action_request, action_type).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn update_balance(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        account_action_request: &AccountActionRequest,
        action_type: &ActionTypeModel,
    ) -> AppResult<()> {
        let amount = &account_action_request.amount;
        let amount_available_balance = action_type
            .available_balance_change
            .calculate_change(*amount);
        let amount_frozen_balance = action_type.frozen_balance_change.calculate_change(*amount);
        let amount_total_income = action_type.total_income_change.calculate_change(*amount);
        let amount_total_expense = action_type.total_expense_change.calculate_change(*amount);
        let account = AccountModel::update_balance(
            &mut **tx,
            account_action_request.user_id,
            account_action_request.asset_type_id,
            amount_available_balance,
            amount_frozen_balance,
            amount_total_income,
            amount_total_expense,
        )
        .await?;
        // 扣减`可用余额/冻结余额`时，不允许`可用余额/冻结余额`为负数
        // 增加`可用余额/冻结余额`时，允许`可用余额/冻结余额`为负数
        // 因为管理员可能直接操作数据库修改用户`可用余额/冻结余额`，所以只在扣减操作才判断
        Self::check_balance_after_update(action_type, &account).await?;
        AccountLogModel::create(
            &mut **tx,
            account.id,
            action_type.id,
            amount_available_balance,
            amount_frozen_balance,
            amount_total_income,
            amount_total_expense,
            account.available_balance,
            account.frozen_balance,
            account.total_income,
            account.total_expense,
            account_action_request.order_number.as_ref(),
            account_action_request.description.as_ref(),
        )
        .await?;
        Ok(())
    }

    pub async fn logs(account_log_request: &AccountLogRequest) -> AppResult<Vec<AccountLogModel>> {
        account_log_request.validate()?;
        let account = AccountModel::find(
            postgres::conn(),
            account_log_request.user_id,
            account_log_request.asset_type_id,
        )
        .await?;

        let tz: chrono_tz::Tz = postgres::pg_session_timezone().parse().unwrap();

        let account_logs = AccountLogModel::query_with_pagination(
            postgres::conn(),
            account.id,
            account_log_request.action_type_id,
            account_log_request
                .start_time
                .as_ref()
                .map(|s| utils::parse_day_boundary(s, tz, utils::DayBoundary::Start))
                .transpose()?,
            account_log_request
                .end_time
                .as_ref()
                .map(|s| utils::parse_day_boundary(s, tz, utils::DayBoundary::End))
                .transpose()?,
            account_log_request.page,
            account_log_request.page_size,
        )
        .await?;
        Ok(account_logs)
    }
}
