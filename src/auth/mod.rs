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

use crate::error::AuthError;
use async_trait::async_trait;

pub mod oidc;

pub struct AuthResponse {
    id: String,
    provider: String,
    email: Option<String>,
}

#[async_trait]
pub trait AuthProvider {
    async fn get_challenge(&self) -> Result<String, AuthError>;
    async fn verify(&self, response: &str) -> Result<AuthResponse, AuthError>;
}
