use crate::LandscapeApp;
use crate::{api::LandscapeApiResp, error::LandscapeApiResult};
use axum::{routing::post, Json, Router};
use landscape_common::route::trace::{
    FlowMatchRequest, FlowMatchResult, FlowVerdictRequest, FlowVerdictResult,
};

pub async fn get_route_paths() -> Router<LandscapeApp> {
    Router::new()
        .route("/route/reset_cache", post(reset_cache))
        .route("/route/trace/flow_match", post(trace_flow_match))
        .route("/route/trace/verdict", post(trace_verdict))
}

async fn reset_cache() -> LandscapeApiResult<()> {
    landscape_ebpf::map_setting::route::cache::recreate_route_lan_cache_inner_map();
    // landscape_ebpf::map_setting::route::cache::recreate_route_wan_cache_inner_map();
    LandscapeApiResp::success(())
}

async fn trace_flow_match(
    Json(req): Json<FlowMatchRequest>,
) -> LandscapeApiResult<FlowMatchResult> {
    let result = landscape_ebpf::map_setting::route::trace_flow_match(req);
    LandscapeApiResp::success(result)
}

async fn trace_verdict(
    Json(req): Json<FlowVerdictRequest>,
) -> LandscapeApiResult<FlowVerdictResult> {
    let result = landscape_ebpf::map_setting::route::trace_flow_verdict(req);
    LandscapeApiResp::success(result)
}
