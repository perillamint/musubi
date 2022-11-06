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

use std::marker::PhantomData;

use super::{AuthProvider, AuthResponse};
use crate::config::OIDCConfig;
use crate::error::AuthError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreGenderClaim, CoreProviderMetadata,
};
use openidconnect::reqwest::async_http_client;
use openidconnect::UserInfoClaims;
use openidconnect::{AdditionalClaims, EmptyAdditionalClaims};
use openidconnect::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, OAuth2TokenResponse,
    RedirectUrl, Scope,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct OIDCAuthContext {
    csrf_token: CsrfToken,
    nonce: Nonce,
}

pub struct OIDCAuthProvider<T> {
    client: CoreClient,
    _marker: PhantomData<T>,
}

impl<T: AdditionalClaims> OIDCAuthProvider<T> {
    pub async fn new(
        client_id: &str,
        client_secret: &str,
        redirect: &str,
        issuer: &str,
    ) -> Result<Self, AuthError> {
        let provider_meta = CoreProviderMetadata::discover_async(
            IssuerUrl::new(issuer.to_string())
                .map_err(|e| AuthError::InvalidConfiguration(e.to_string()))?,
            async_http_client,
        )
        .await
        .map_err(|e| AuthError::InvalidConfiguration(e.to_string()))?;

        let client = CoreClient::from_provider_metadata(
            provider_meta,
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect.to_string())
                .map_err(|e| AuthError::InvalidConfiguration(e.to_string()))?,
        );

        Ok(OIDCAuthProvider {
            client,
            _marker: PhantomData,
        })
    }
}

#[async_trait]
impl<T: AdditionalClaims + Send + Sync + Clone> AuthProvider<OIDCAuthContext, String, T>
    for OIDCAuthProvider<T>
{
    async fn get_challenge(&self) -> Result<(String, OIDCAuthContext), AuthError> {
        let (auth_url, csrf_token, nonce) = self
            .client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("openid".to_owned()))
            .add_scope(Scope::new("email".to_owned()))
            .add_scope(Scope::new("musubi".to_owned()))
            .url();

        Ok((auth_url.to_string(), OIDCAuthContext { csrf_token, nonce }))
    }

    async fn verify(
        &self,
        context: OIDCAuthContext,
        response: String,
    ) -> Result<AuthResponse<T>, AuthError> {
        let token_response = self
            .client
            .exchange_code(AuthorizationCode::new(response))
            .request_async(async_http_client)
            .await
            .map_err(|e| AuthError::RemoteServiceError(e.to_string()))?;

        let id_token = match token_response.extra_fields().id_token() {
            Some(x) => x,
            None => {
                return Err(AuthError::RemoteServiceError(
                    "Token endpoint did not return id_token".to_string(),
                ))
            }
        };

        // TODO: Implement CSRF check

        let _claims = match id_token.claims(&self.client.id_token_verifier(), &context.nonce) {
            Ok(x) => x,
            Err(e) => return Err(AuthError::InvalidCredential(e.to_string())),
        };

        let userinfo_claims: UserInfoClaims<T, CoreGenderClaim> = self
            .client
            .user_info(token_response.access_token().to_owned(), None)
            .map_err(|e| AuthError::RemoteServiceError(e.to_string()))?
            .request_async(async_http_client)
            .await
            .map_err(|e| AuthError::RemoteServiceError(e.to_string()))?;

        let email = userinfo_claims.email().map(|x| x.to_string());
        let id = userinfo_claims.subject().to_string();

        // Everything goes well...
        Ok(AuthResponse {
            id,
            provider: "".to_string(),
            email,
            permission: Some(userinfo_claims.additional_claims().clone()),
        })
    }
}
