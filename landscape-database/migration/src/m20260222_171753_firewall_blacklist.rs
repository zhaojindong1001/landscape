use sea_orm_migration::prelude::*;

use crate::tables::firewall_blacklist::FirewallBlacklistConfigs;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FirewallBlacklistConfigs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FirewallBlacklistConfigs::Id).uuid().primary_key())
                    .col(ColumnDef::new(FirewallBlacklistConfigs::Enable).boolean().not_null())
                    .col(ColumnDef::new(FirewallBlacklistConfigs::Source).json().not_null())
                    .col(
                        ColumnDef::new(FirewallBlacklistConfigs::Remark)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(FirewallBlacklistConfigs::UpdateAt)
                            .double()
                            .not_null()
                            .default(0.0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(FirewallBlacklistConfigs::Table).to_owned()).await
    }
}
