use crate::handler;
use axum::{
    Router,
    routing::{get, post},
};
use axum_kit::middleware::{cors, request_id, trace, trace_body};
use tower::ServiceBuilder;

pub fn init() -> Router {
    Router::new()
        // 获取资产类型
        .route("/assets", get(handler::asset_type::list))
        // 获取账户操作类型
        .route("/actions", get(handler::action_type::list))
        // 添加资产账户
        .route("/accounts/new", post(handler::account::create))
        // 获取资产账户信息
        .route("/accounts/info", post(handler::account::info))
        // 获取某`user_id`所有资产账户信息
        .route("/accounts/infos", post(handler::account::infos))
        // 资产账户操作
        .route("/accounts/actions", post(handler::account::actions))
        // 资产账户操作记录
        .route("/accounts/logs", post(handler::account::logs))
        .layer(
            ServiceBuilder::new()
                .layer(request_id::set_request_id())
                .layer(request_id::propagate_request_id())
                .layer(trace::trace())
                .layer(cors::cors())
                .layer(trace_body::trace_body()),
        )
}
