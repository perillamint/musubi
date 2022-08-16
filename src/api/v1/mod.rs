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

use actix_web::{get, web, HttpResponse, Responder, Result, Scope};

mod interface;
mod keys;

use super::types::{APIResponse, JsonAPIResponse};

#[get("/")]
async fn index() -> Result<JsonAPIResponse<String>> {
    Ok(web::Json(APIResponse {
        code: 0,
        error: None,
        message: Some("Hello, musubi!".to_string()),
    }))
}

pub fn get_service() -> Scope {
    web::scope("/v1")
        .service(keys::get_service())
        .service(interface::get_service())
}
