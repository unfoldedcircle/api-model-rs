// Copyright (c) 2023 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Shared models between Core- & Integration-API

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod intg;
pub mod settings;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Oauth2Token {
    /// The access token issued by the authorization server.
    pub access_token: String,
    /// The type of the token issued. E.g. `Bearer`.
    pub token_type: String,
    /// The time period (in seconds) for which the `access_token` is valid.
    pub expires_in: Option<u64>,
    /// Injected value by the core when the `access_token` expires, based on `expires_in` and the time of the authorization request.
    pub expires_at: Option<DateTime<Utc>>,
    /// The refresh token, which can be used to obtain new access tokens using the same authorization grant.
    pub refresh_token: Option<String>,
    /// A space-separated list of scopes which have been granted for this `access_token`.
    pub scope: Option<String>,
}
