use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{flow::mark::FlowMark, net::MacAddr};

// ===== Step 1: Flow Match =====

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "common/route_trace.d.ts")]
pub struct FlowMatchRequest {
    #[serde(default)]
    #[ts(optional)]
    pub src_ipv4: Option<Ipv4Addr>,
    #[serde(default)]
    #[ts(optional)]
    pub src_ipv6: Option<Ipv6Addr>,
    pub src_mac: Option<MacAddr>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "common/route_trace.d.ts")]
pub struct FlowMatchResult {
    pub flow_id_by_mac: Option<u32>,
    pub flow_id_by_ip: Option<u32>,
    pub effective_flow_id: u32,
}

// ===== Step 2: Flow Verdict =====

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "common/route_trace.d.ts")]
pub struct FlowVerdictRequest {
    pub flow_id: u32,
    #[serde(default)]
    #[ts(optional)]
    pub src_ipv4: Option<Ipv4Addr>,
    #[serde(default)]
    #[ts(optional)]
    pub src_ipv6: Option<Ipv6Addr>,
    pub dst_ips: Vec<IpAddr>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "common/route_trace.d.ts")]
pub struct FlowVerdictResult {
    pub verdicts: Vec<SingleVerdictResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "common/route_trace.d.ts")]
pub struct SingleVerdictResult {
    pub dst_ip: IpAddr,
    pub ip_rule_match: Option<FlowRuleMatchResult>,
    pub dns_rule_match: Option<FlowRuleMatchResult>,
    pub effective_mark: FlowMark,
    pub has_cache: bool,
    pub cached_mark: Option<u32>,
    pub cache_consistent: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "common/route_trace.d.ts")]
pub struct FlowRuleMatchResult {
    pub mark: FlowMark,
    pub priority: u16,
}
