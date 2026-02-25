pub mod repository;

use crate::{config::FlowId, error::LdError};

/// 干净的数据库存储接口，不依赖任何 ORM 类型
#[async_trait::async_trait]
pub trait LandscapeStore: Send + Sync {
    type Data: Send + Sync + std::fmt::Debug;
    type Id: Send + Sync + std::fmt::Debug;

    async fn set(&self, config: Self::Data) -> Result<Self::Data, LdError>;
    async fn list(&self) -> Result<Vec<Self::Data>, LdError>;
    async fn delete(&self, id: Self::Id) -> Result<(), LdError>;
    async fn find_by_id(&self, id: Self::Id) -> Result<Option<Self::Data>, LdError>;
    async fn find_by_ids(&self, ids: Vec<Self::Id>) -> Vec<Self::Data>;
}

/// 支持 Flow 查询的存储接口
#[async_trait::async_trait]
pub trait LandscapeFlowStore: LandscapeStore {
    async fn find_by_flow_id(&self, flow_id: FlowId) -> Result<Vec<Self::Data>, LdError>;
}
