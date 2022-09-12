/*
 * SPDX-FileCopyrightText: 2022 perillamint
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use sea_orm_migration::prelude::*;

use super::m20220902_154034_create_peer_table::Peer;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FKEY_IP_TABLE_PEER_ID: &str = "fkey_ip_table_peer_id";
const IDX_IP_TABLE_PEER_ID: &str = "idx_ip_table_peer_id";
const IDX_IP_TABLE_ADDRESS: &str = "idx_ip_table_address";
const IDX_IP_TABLE_CREATED_AT: &str = "idx_ip_table_created_at";
const IDX_IP_TABLE_UPDATED_AT: &str = "idx_ip_table_updated_at";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ip::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Ip::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Ip::PeerId).uuid().not_null())
                    .col(ColumnDef::new(Ip::Address).string().not_null())
                    .col(
                        ColumnDef::new(Ip::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ip::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name(FKEY_IP_TABLE_PEER_ID)
                            .from(Ip::Table, Ip::PeerId)
                            .to(Peer::Table, Peer::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_IP_TABLE_PEER_ID)
                    .table(Ip::Table)
                    .col(Ip::PeerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_IP_TABLE_ADDRESS)
                    .table(Ip::Table)
                    .col(Ip::Address)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_IP_TABLE_CREATED_AT)
                    .table(Ip::Table)
                    .col(Ip::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_IP_TABLE_UPDATED_AT)
                    .table(Ip::Table)
                    .col(Ip::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ip::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Ip {
    Table,
    Id,
    PeerId,
    Address,
    CreatedAt,
    UpdatedAt,
}
