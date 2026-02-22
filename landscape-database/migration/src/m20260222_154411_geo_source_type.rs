use sea_orm_migration::{prelude::*, sea_orm::FromQueryResult};
use uuid::Uuid;

use crate::tables::geo::{GeoIpConfigs, GeoSiteConfigs};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use sea_orm_migration::sea_orm::{ConnectionTrait, TransactionTrait};

        let db = manager.get_connection();
        let txn = db.begin().await?;
        let builder = manager.get_database_backend();

        // ── GeoSite ──

        // 1. Read existing rows
        let select = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("update_at"),
                Alias::new("url"),
                Alias::new("name"),
                Alias::new("enable"),
                Alias::new("next_update_at"),
                Alias::new("geo_keys"),
            ])
            .from(Alias::new("geo_site_configs"))
            .to_owned();

        let site_rows: Vec<OldGeoSiteRow> =
            OldGeoSiteRow::find_by_statement(builder.build(&select)).all(&txn).await?;

        // 2. Drop and recreate table with source column
        txn.execute(builder.build(&Table::drop().table(GeoSiteConfigs::Table).to_owned())).await?;

        txn.execute(
            builder.build(
                &Table::create()
                    .table(GeoSiteConfigs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GeoSiteConfigs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(GeoSiteConfigs::UpdateAt).double().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::Enable).boolean().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::Source).json().not_null())
                    .to_owned(),
            ),
        )
        .await?;

        // 3. Migrate data
        if !site_rows.is_empty() {
            let mut insert = Query::insert()
                .into_table(GeoSiteConfigs::Table)
                .columns([
                    GeoSiteConfigs::Id,
                    GeoSiteConfigs::UpdateAt,
                    GeoSiteConfigs::Name,
                    GeoSiteConfigs::Enable,
                    GeoSiteConfigs::Source,
                ])
                .to_owned();

            for row in site_rows {
                let geo_keys: Vec<String> = serde_json::from_str(&row.geo_keys).unwrap_or_default();

                let source = serde_json::json!({
                    "t": "url",
                    "url": row.url,
                    "next_update_at": row.next_update_at,
                    "geo_keys": geo_keys,
                });

                insert.values_panic([
                    row.id.into(),
                    row.update_at.into(),
                    row.name.into(),
                    row.enable.into(),
                    source.into(),
                ]);
            }

            txn.execute(builder.build(&insert)).await?;
        }

        // ── GeoIp ──

        // 1. Read existing rows
        let select = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("update_at"),
                Alias::new("url"),
                Alias::new("name"),
                Alias::new("enable"),
                Alias::new("next_update_at"),
            ])
            .from(Alias::new("geo_ip_configs"))
            .to_owned();

        let ip_rows: Vec<OldGeoIpRow> =
            OldGeoIpRow::find_by_statement(builder.build(&select)).all(&txn).await?;

        // 2. Drop and recreate table with source column
        txn.execute(builder.build(&Table::drop().table(GeoIpConfigs::Table).to_owned())).await?;

        txn.execute(
            builder.build(
                &Table::create()
                    .table(GeoIpConfigs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GeoIpConfigs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(GeoIpConfigs::UpdateAt).double().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::Enable).boolean().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::Source).json().not_null())
                    .to_owned(),
            ),
        )
        .await?;

        // 3. Migrate data
        if !ip_rows.is_empty() {
            let mut insert = Query::insert()
                .into_table(GeoIpConfigs::Table)
                .columns([
                    GeoIpConfigs::Id,
                    GeoIpConfigs::UpdateAt,
                    GeoIpConfigs::Name,
                    GeoIpConfigs::Enable,
                    GeoIpConfigs::Source,
                ])
                .to_owned();

            for row in ip_rows {
                let source = serde_json::json!({
                    "t": "url",
                    "url": row.url,
                    "next_update_at": row.next_update_at,
                });

                insert.values_panic([
                    row.id.into(),
                    row.update_at.into(),
                    row.name.into(),
                    row.enable.into(),
                    source.into(),
                ]);
            }

            txn.execute(builder.build(&insert)).await?;
        }

        txn.commit().await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use sea_orm_migration::sea_orm::{ConnectionTrait, TransactionTrait};

        let db = manager.get_connection();
        let txn = db.begin().await?;
        let builder = manager.get_database_backend();

        // ── GeoSite rollback ──

        let select = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("update_at"),
                Alias::new("name"),
                Alias::new("enable"),
                Alias::new("source"),
            ])
            .from(Alias::new("geo_site_configs"))
            .to_owned();

        let site_rows: Vec<NewGeoSiteRow> =
            NewGeoSiteRow::find_by_statement(builder.build(&select)).all(&txn).await?;

        txn.execute(builder.build(&Table::drop().table(GeoSiteConfigs::Table).to_owned())).await?;

        txn.execute(
            builder.build(
                &Table::create()
                    .table(GeoSiteConfigs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GeoSiteConfigs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(GeoSiteConfigs::UpdateAt).double().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::Url).string().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::Enable).boolean().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::NextUpdateAt).double().not_null())
                    .col(ColumnDef::new(GeoSiteConfigs::GeoKeys).json().not_null())
                    .to_owned(),
            ),
        )
        .await?;

        if !site_rows.is_empty() {
            let mut insert = Query::insert()
                .into_table(GeoSiteConfigs::Table)
                .columns([
                    GeoSiteConfigs::Id,
                    GeoSiteConfigs::UpdateAt,
                    GeoSiteConfigs::Url,
                    GeoSiteConfigs::Name,
                    GeoSiteConfigs::Enable,
                    GeoSiteConfigs::NextUpdateAt,
                    GeoSiteConfigs::GeoKeys,
                ])
                .to_owned();

            for row in site_rows {
                let source: serde_json::Value =
                    serde_json::from_str(&row.source).unwrap_or_default();
                let url = source["url"].as_str().unwrap_or("").to_string();
                let next_update_at = source["next_update_at"].as_f64().unwrap_or(0.0);
                let geo_keys = source.get("geo_keys").cloned().unwrap_or(serde_json::json!([]));

                insert.values_panic([
                    row.id.into(),
                    row.update_at.into(),
                    url.into(),
                    row.name.into(),
                    row.enable.into(),
                    next_update_at.into(),
                    geo_keys.into(),
                ]);
            }

            txn.execute(builder.build(&insert)).await?;
        }

        // ── GeoIp rollback ──

        let select = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("update_at"),
                Alias::new("name"),
                Alias::new("enable"),
                Alias::new("source"),
            ])
            .from(Alias::new("geo_ip_configs"))
            .to_owned();

        let ip_rows: Vec<NewGeoIpRow> =
            NewGeoIpRow::find_by_statement(builder.build(&select)).all(&txn).await?;

        txn.execute(builder.build(&Table::drop().table(GeoIpConfigs::Table).to_owned())).await?;

        txn.execute(
            builder.build(
                &Table::create()
                    .table(GeoIpConfigs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GeoIpConfigs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(GeoIpConfigs::UpdateAt).double().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::Url).string().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::Enable).boolean().not_null())
                    .col(ColumnDef::new(GeoIpConfigs::NextUpdateAt).double().not_null())
                    .to_owned(),
            ),
        )
        .await?;

        if !ip_rows.is_empty() {
            let mut insert = Query::insert()
                .into_table(GeoIpConfigs::Table)
                .columns([
                    GeoIpConfigs::Id,
                    GeoIpConfigs::UpdateAt,
                    GeoIpConfigs::Url,
                    GeoIpConfigs::Name,
                    GeoIpConfigs::Enable,
                    GeoIpConfigs::NextUpdateAt,
                ])
                .to_owned();

            for row in ip_rows {
                let source: serde_json::Value =
                    serde_json::from_str(&row.source).unwrap_or_default();
                let url = source["url"].as_str().unwrap_or("").to_string();
                let next_update_at = source["next_update_at"].as_f64().unwrap_or(0.0);

                insert.values_panic([
                    row.id.into(),
                    row.update_at.into(),
                    url.into(),
                    row.name.into(),
                    row.enable.into(),
                    next_update_at.into(),
                ]);
            }

            txn.execute(builder.build(&insert)).await?;
        }

        txn.commit().await?;
        Ok(())
    }
}

#[derive(FromQueryResult)]
struct OldGeoSiteRow {
    id: Uuid,
    update_at: f64,
    url: String,
    name: String,
    enable: bool,
    next_update_at: f64,
    geo_keys: String,
}

#[derive(FromQueryResult)]
struct OldGeoIpRow {
    id: Uuid,
    update_at: f64,
    url: String,
    name: String,
    enable: bool,
    next_update_at: f64,
}

#[derive(FromQueryResult)]
struct NewGeoSiteRow {
    id: Uuid,
    update_at: f64,
    name: String,
    enable: bool,
    source: String,
}

#[derive(FromQueryResult)]
struct NewGeoIpRow {
    id: Uuid,
    update_at: f64,
    name: String,
    enable: bool,
    source: String,
}
