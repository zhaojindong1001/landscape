use landscape_common::{
    database::{repository::Repository, LandscapeDBTrait},
    firewall::blacklist::FirewallBlacklistConfig,
};
use sea_orm::DatabaseConnection;

use crate::{firewall_blacklist::entity::FirewallBlacklistConfigEntity, DBId};

use super::entity::{FirewallBlacklistConfigActiveModel, FirewallBlacklistConfigModel};

#[derive(Clone)]
pub struct FirewallBlacklistRepository {
    db: DatabaseConnection,
}

impl FirewallBlacklistRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl LandscapeDBTrait for FirewallBlacklistRepository {}

#[async_trait::async_trait]
impl Repository for FirewallBlacklistRepository {
    type Model = FirewallBlacklistConfigModel;
    type Entity = FirewallBlacklistConfigEntity;
    type ActiveModel = FirewallBlacklistConfigActiveModel;
    type Data = FirewallBlacklistConfig;
    type Id = DBId;

    fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
