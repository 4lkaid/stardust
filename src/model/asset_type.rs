use axum_kit::AppResult;
use serde::Serialize;
use sqlx::{
    PgExecutor,
    types::chrono::{DateTime, Utc},
};

#[derive(Serialize)]
pub struct AssetTypeModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub is_active: bool,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}

impl AssetTypeModel {
    pub async fn fetch_all(executor: impl PgExecutor<'_>) -> AppResult<Vec<Self>> {
        let asset_types: Vec<Self> = sqlx::query_as!(
            Self,
            r#"select
                id,
                name,
                description,
                is_active,
                created_at,
                updated_at
            from
                asset_type
            where
                is_active = true"#
        )
        .fetch_all(executor)
        .await?;
        Ok(asset_types)
    }
}
