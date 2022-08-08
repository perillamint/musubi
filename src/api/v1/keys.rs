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

use super::types::{APIResponse, JsonAPIResponse, Key, Peer};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder, Result, Scope};

#[get("/")]
async fn get_keys() -> Result<JsonAPIResponse<String>> {
    Ok(web::Json(APIResponse {
        code: 0,
        error: None,
        message: Some("Hello World".to_string()),
    }))
}

#[post("/")]
async fn add_key(req: HttpRequest, key: web::Json<Peer>) -> Result<JsonAPIResponse<String>> {
    Ok(web::Json(APIResponse {
        code: 0,
        error: None,
        message: Some(String::from("OK")),
    }))
}

#[delete("/{key_id}")]
async fn delete_key(req: HttpRequest) -> Result<JsonAPIResponse<Key>> {
    let key = Key {
        pubkey: "".to_string(),
        privkey: None,
        psk: None,
    };

    Ok(web::Json(APIResponse {
        code: 0,
        error: None,
        message: Some(key),
    }))
}

pub fn get_service() -> Scope {
    web::scope("/keys").service(get_keys).service(add_key)
}
