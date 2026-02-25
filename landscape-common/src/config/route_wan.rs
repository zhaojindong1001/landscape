use serde::{Deserialize, Serialize};

use crate::utils::time::get_f64_timestamp;
use crate::{database::repository::LandscapeDBStore, store::storev2::LandscapeStore};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RouteWanServiceConfig {
    pub iface_name: String,
    pub enable: bool,
    #[serde(default = "get_f64_timestamp")]
    #[cfg_attr(feature = "openapi", schema(required = true))]
    pub update_at: f64,
}

impl LandscapeStore for RouteWanServiceConfig {
    fn get_store_key(&self) -> String {
        self.iface_name.clone()
    }
}

impl LandscapeDBStore<String> for RouteWanServiceConfig {
    fn get_id(&self) -> String {
        self.iface_name.clone()
    }
}

impl super::iface::ZoneAwareConfig for RouteWanServiceConfig {
    fn iface_name(&self) -> &str {
        &self.iface_name
    }
    fn zone_requirement() -> super::iface::ZoneRequirement {
        super::iface::ZoneRequirement::WanOrPpp
    }
    fn service_kind() -> super::iface::ServiceKind {
        super::iface::ServiceKind::RouteWan
    }
}
