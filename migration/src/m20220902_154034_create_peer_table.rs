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

use super::m20220902_153334_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FKEY_PEER_TABLE_OWNER_ID: &str = "fkey_peer_table_owner_id";
const IDX_PEER_TABLE_OWNER_ID: &str = "idx_peer_table_owner_id";
const IDX_PEER_TABLE_PUBKEY: &str = "idx_peer_table_pubkey";
const IDX_PEER_TABLE_STATUS: &str = "idx_peer_table_status";
const IDX_PEER_TABLE_CREATED_AT: &str = "idx_peer_table_created_at";
const IDX_PEER_TABLE_UPDATED_AT: &str = "idx_peer_table_updated_at";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Peer::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Peer::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Peer::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Peer::Pubkey).string().not_null())
                    .col(ColumnDef::new(Peer::Psk).string().not_null())
                    .col(ColumnDef::new(Peer::Status).string().not_null())
                    .col(
                        ColumnDef::new(Peer::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Peer::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name(FKEY_PEER_TABLE_OWNER_ID)
                            .from(Peer::Table, Peer::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_PEER_TABLE_OWNER_ID)
                    .table(Peer::Table)
                    .col(Peer::OwnerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_PEER_TABLE_PUBKEY)
                    .table(Peer::Table)
                    .col(Peer::Pubkey)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_PEER_TABLE_STATUS)
                    .table(Peer::Table)
                    .col(Peer::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_PEER_TABLE_CREATED_AT)
                    .table(Peer::Table)
                    .col(Peer::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_PEER_TABLE_UPDATED_AT)
                    .table(Peer::Table)
                    .col(Peer::UpdatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Peer::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum Peer {
    Table,
    Id,
    OwnerId,
    Pubkey,
    Psk,
    Status,
    CreatedAt,
    UpdatedAt,
}
