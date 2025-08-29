use axum_kit::AppResult;
use serde::Serialize;
use sqlx::{
    PgExecutor,
    types::{
        Decimal,
        chrono::{DateTime, Utc},
    },
};

#[derive(Serialize)]
pub struct AccountModel {
    pub id: i32,
    pub user_id: i32,
    pub asset_type_id: i32,
    pub available_balance: Decimal,
    pub frozen_balance: Decimal,
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub is_active: bool,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}

impl AccountModel {
    pub async fn create(
        executor: impl PgExecutor<'_>,
        user_id: i32,
        asset_type_id: i32,
    ) -> AppResult<Self> {
        let account = sqlx::query_as!(
            Self,
            r#"insert into account(user_id, asset_type_id, is_active)
                values ($1, $2, true)
            returning
                id,
                user_id,
                asset_type_id,
                available_balance,
                frozen_balance,
                total_income,
                total_expense,
                is_active,
                created_at,
                updated_at"#,
            user_id,
            asset_type_id
        )
        .fetch_one(executor)
        .await?;
        Ok(account)
    }

    pub async fn find(
        executor: impl PgExecutor<'_>,
        user_id: i32,
        asset_type_id: i32,
    ) -> AppResult<Self> {
        let account = sqlx::query_as!(
            Self,
            r#"select
                id,
                user_id,
                asset_type_id,
                available_balance,
                frozen_balance,
                total_income,
                total_expense,
                is_active,
                created_at,
                updated_at
            from
                account
            where
                user_id = $1
                and asset_type_id = $2"#,
            user_id,
            asset_type_id
        )
        .fetch_one(executor)
        .await?;
        Ok(account)
    }

    // 事务内查询并锁定账户（行级锁，仅在事务内生效）
    pub async fn find_for_update(
        executor: impl PgExecutor<'_>,
        user_id: i32,
        asset_type_id: i32,
    ) -> AppResult<Self> {
        let account = sqlx::query_as!(
            Self,
            r#"select
                id,
                user_id,
                asset_type_id,
                available_balance,
                frozen_balance,
                total_income,
                total_expense,
                is_active,
                created_at,
                updated_at
            from
                account
            where
                user_id = $1
                and asset_type_id = $2
            for update"#,
            user_id,
            asset_type_id
        )
        .fetch_one(executor)
        .await?;
        Ok(account)
    }

    pub async fn find_multiple(
        executor: impl PgExecutor<'_>,
        user_id: i32,
        asset_type_ids: Vec<i32>,
    ) -> AppResult<Vec<Self>> {
        let accounts = sqlx::query_as!(
            Self,
            r#"select
                id,
                user_id,
                asset_type_id,
                available_balance,
                frozen_balance,
                total_income,
                total_expense,
                is_active,
                created_at,
                updated_at
            from
                account
            where
                user_id = $1
                and asset_type_id = any($2)"#,
            user_id,
            &asset_type_ids
        )
        .fetch_all(executor)
        .await?;
        Ok(accounts)
    }

    pub async fn update_balance(
        executor: impl PgExecutor<'_>,
        user_id: i32,
        asset_type_id: i32,
        amount_available_balance: Decimal,
        amount_frozen_balance: Decimal,
        amount_total_income: Decimal,
        amount_total_expense: Decimal,
    ) -> AppResult<Self> {
        let account = sqlx::query_as!(
            Self,
            r#"update account
                set available_balance = available_balance + $3,
                frozen_balance = frozen_balance + $4,
                total_income = total_income + $5,
                total_expense = total_expense + $6,
                updated_at = now()
            where
                user_id = $1
                and asset_type_id = $2
            returning
                id,
                user_id,
                asset_type_id,
                available_balance,
                frozen_balance,
                total_income,
                total_expense,
                is_active,
                created_at,
                updated_at"#,
            user_id,
            asset_type_id,
            amount_available_balance,
            amount_frozen_balance,
            amount_total_income,
            amount_total_expense,
        )
        .fetch_one(executor)
        .await?;
        Ok(account)
    }

    // 资产账户是否存在
    pub async fn is_exists(
        executor: impl PgExecutor<'_>,
        user_id: i32,
        asset_type_id: i32,
    ) -> bool {
        if let Ok(Some(exists)) = sqlx::query_scalar!(
            r#"select exists(select 1 from account where user_id = $1 and asset_type_id = $2)"#,
            user_id,
            asset_type_id
        )
        .fetch_one(executor)
        .await
        {
            return exists;
        }
        false
    }

    // 资产账户是否启用
    // #[allow(dead_code)]
    // pub async fn is_active(
    //     executor: impl PgExecutor<'_>,
    //     user_id: i32,
    //     asset_type_id: i32,
    // ) -> bool {
    //     if let Ok(Some(exists)) = sqlx::query_scalar!(
    //         r#"select exists(select 1 from account where user_id = $1 and asset_type_id = $2 and is_active = true)"#,
    //         user_id,
    //         asset_type_id
    //     )
    //     .fetch_one(executor)
    //     .await
    //     {
    //         return exists;
    //     }
    //     false
    // }
}
