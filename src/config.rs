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

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct HTTPConfig {
    pub bind: String,
    pub port: u16,
}

impl Default for HTTPConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Database {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/musubi".to_string(),
            max_connections: 100,
            min_connections: 5,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct OIDCConfig {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub http: HTTPConfig,
    #[serde(default)]
    pub database: Database,
    pub oidc: OIDCConfig,
}

pub(crate) fn parse_toml(tomlstr: &str) -> Config {
    let cfg: Config = toml::from_str(tomlstr).expect("Invalid config file");
    cfg
}

pub(crate) fn read_config(cfgpath: &str) -> Config {
    match fs::read_to_string(cfgpath) {
        Ok(x) => parse_toml(&x),
        Err(_) => panic!("Config file not found!"),
    }
}
