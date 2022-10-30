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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credential: {0}")]
    InvalidCredential(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Failed to get response from remote authentication server.")]
    RemoteConnectionFailure(#[from] reqwest::Error),
    #[error("Failed to get proper response from remote authentication server: {0}")]
    RemoteServiceError(String),
    #[error("Failed to vaildate response from remote authentication server: {0}")]
    InvalidResponse(String),
    #[error("Unknown internal error. BAD!")]
    UnknownError,
}
