pub enum DnsEvent {
    RuleUpdated { flow_id: Option<u32> },
    GeositeUpdated,
    FlowUpdated,
}

#[derive(Clone, Debug)]
pub enum DstIpEvent {
    GeoIpUpdated,
}
