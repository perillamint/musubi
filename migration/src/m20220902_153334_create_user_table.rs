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

#[derive(DeriveMigrationName)]
pub struct Migration;

const IDX_USER_TABLE_EXTERNAL_ID: &str = "idx_user_table_external_id";
const IDX_USER_TABLE_EMAIL: &str = "idx_user_table_email";
const IDX_USER_TABLE_STATUS: &str = "idx_user_table_status";
const IDX_USER_TABLE_CREATED_AT: &str = "idx_user_table_created_at";
const IDX_USER_TABLE_UPDATED_AT: &str = "idx_user_table_updated_at";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::ExternalId)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Status).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_TABLE_EXTERNAL_ID)
                    .table(User::Table)
                    .col(User::ExternalId)
                    .index_type(IndexType::Hash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_TABLE_EMAIL)
                    .table(User::Table)
                    .col(User::Email)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_TABLE_STATUS)
                    .table(User::Table)
                    .col(User::Status)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_TABLE_CREATED_AT)
                    .table(User::Table)
                    .col(User::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_USER_TABLE_UPDATED_AT)
                    .table(User::Table)
                    .col(User::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(User::Table)
                    .name(IDX_USER_TABLE_EMAIL)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(User::Table)
                    .name(IDX_USER_TABLE_EXTERNAL_ID)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(User::Table)
                    .name(IDX_USER_TABLE_STATUS)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(User::Table)
                    .name(IDX_USER_TABLE_CREATED_AT)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(User::Table)
                    .name(IDX_USER_TABLE_UPDATED_AT)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum User {
    Table,
    Id,
    ExternalId,
    Email,
    Status,
    CreatedAt,
    UpdatedAt,
}
