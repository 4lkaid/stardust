mod constant;
mod handler;
mod model;
mod request;
mod route;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _worker_guard = axum_kit::bootstrap::Application::default("config.toml")?
        .with_router(route::api::init)
        .before_run(|| {
            tokio::spawn(async move {
                service::asset_type::AssetTypeService::init().await?;
                service::action_type::ActionTypeService::init().await?;
                Ok(())
            })
        })
        .run()
        .await?;
    Ok(())
}
