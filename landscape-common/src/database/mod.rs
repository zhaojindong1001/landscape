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

    /// 只读冲突检查。
    /// - 记录不存在 → Ok(None)
    /// - 记录存在且 update_at 匹配 → Ok(Some(旧配置))
    /// - 记录存在但 update_at 不匹配 → Err(ConfigConflict)
    async fn check_conflict(&self, config: &Self::Data) -> Result<Option<Self::Data>, LdError>;

    /// 乐观锁 set：检查 update_at + 刷新时间戳 + 写入
    async fn checked_set(&self, config: Self::Data) -> Result<Self::Data, LdError>;
}

/// 支持 Flow 查询的存储接口
#[async_trait::async_trait]
pub trait LandscapeFlowStore: LandscapeStore {
    async fn find_by_flow_id(&self, flow_id: FlowId) -> Result<Vec<Self::Data>, LdError>;
}
