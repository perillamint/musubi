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

use super::AuthResponse;
use crate::error::AuthError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OIDCAuthContext {
    //
}

pub struct OIDCAuthProvider {}

impl OIDCAuthProvider {
    pub fn new(client_id: &str, client_secret: &str, redirect: &str, issuer: &str) -> Self {
        OIDCAuthProvider {}
    }
}

#[async_trait]
impl super::AuthProvider for OIDCAuthProvider {
    async fn get_challenge(&self) -> Result<String, AuthError> {
        todo!("implement me.")
    }

    async fn verify(&self, response: &str) -> Result<AuthResponse, AuthError> {
        todo!("implement me!");
    }
}
