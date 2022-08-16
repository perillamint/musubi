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

use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Interface {
    name: String,
    listen_port: i32,
    key: Key,
}

#[derive(Serialize, Deserialize)]
pub struct Peer {
    pubkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) psk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) endpoint: Option<String>,
    pub(crate) allowed_ips: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) keepalive: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct Key {
    pub pubkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psk: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<T>,
}

pub trait IntoResult<T> {
    /// Convert APIResponse<T> to actix_web Result<JsonAPIResponse<T>>
    fn into_result(self) -> Result<JsonAPIResponse<T>, actix_web::Error>;
}

impl<T: for<'a> Deserialize<'a> + Serialize> IntoResult<T> for APIResponse<T> {
    fn into_result(self) -> Result<Json<Self>, actix_web::Error> {
        match self.error {
            None => Ok(Json(self)),
            Some(_) => Err(actix_web::error::InternalError::from_response(
                "",
                HttpResponse::Ok()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(self),
            )
            .into()),
        }
    }
}

pub type JsonAPIResponse<T> = Json<APIResponse<T>>;
