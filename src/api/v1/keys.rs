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

use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder, Result, Scope};
use serde::Serialize;
use super::types::{Peer};

#[derive(Serialize)]
struct APIHello {
    msg: String
}

#[get("/")]
async fn get_keys() -> Result<impl Responder> {
    let hello = APIHello {
        msg: String::from("Hello, world!"),
    };
    Ok(web::Json(hello))
}

#[post("/")]
async fn add_key(req: HttpRequest, key: web::Json<Peer>) -> impl Responder {
    HttpResponse::Ok().body("TODO: Implement me")
}

#[delete("/{key_id}")]
async fn delete_key(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("TODO: Implement me")
}

pub fn get_service() -> Scope {
    web::scope("/keys").service(get_keys).service(add_key)
}
