use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum FirewallBlacklistConfigs {
    Table,
    Id,
    Enable,
    Source,
    Remark,
    UpdateAt,
}
