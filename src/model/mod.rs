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

/// Paging query parameters.
///
/// All parameters are optional, defaults are used if omitted.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Paging {
    /// Page number, 1-based. If omitted, the first page is returned
    #[validate(range(min = 1, message = "Invalid page number"))]
    page: Option<u32>,
    /// Number of items returned per page. You can set a custom limit up to 100.
    #[validate(range(min = 1, max = 100, message = "Invalid limit number"))]
    limit: Option<u32>,
}

impl Default for Paging {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
        }
    }
}

impl Paging {
    /// Create a new Paging object.
    ///
    /// # Arguments
    ///
    /// * `page`: Page number, 1-based.
    /// * `limit`: Number of items returned per page. Max 100.
    ///
    /// returns: Paging
    pub fn new(page: impl Into<Option<u32>>, limit: impl Into<Option<u32>>) -> Paging {
        Paging {
            page: page.into(),
            limit: limit.into(),
        }
    }

    /// Page number, 1-based
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    /// Overall item start offset, 0-based
    pub fn offset(&self) -> u32 {
        self.limit.unwrap_or(1) * (self.page() - 1)
    }

    /// Number of items returned per page. Defaults to 10 if not specified.
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(10)
    }
}

/// Paging information in response message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    /// Total number of items if known.
    pub count: Option<u32>,
    /// Number of items returned.
    pub limit: u32,
    /// Current page number. 1-based.
    pub page: u32,
}

impl Pagination {
    pub fn new(count: u32, limit: u32, page: u32) -> Self {
        Self {
            count: Some(count),
            limit,
            page,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paging_default() {
        let p = Paging::default();
        assert_eq!(1, p.page());
        assert_eq!(10, p.limit());
        assert_eq!(0, p.offset());
    }

    #[test]
    fn paging_new() {
        // Variation: Some(number)
        let p = Paging::new(Some(2), Some(20));
        assert_eq!(2, p.page());
        assert_eq!(20, p.limit());
        assert_eq!(20, p.offset());

        // Variation: number
        let p = Paging::new(3, 30);
        assert_eq!(3, p.page());
        assert_eq!(30, p.limit());
        assert_eq!(60, p.offset());

        // Variation: None
        let p = Paging::new(None, None);
        assert_eq!(1, p.page());
        assert_eq!(10, p.limit());
        assert_eq!(0, p.offset());

        // Variation: Mixed types
        let p = Paging::new(Some(4), None);
        assert_eq!(4, p.page());
        assert_eq!(10, p.limit());
        assert_eq!(3, p.offset());
    }

    #[test]
    fn paging_offset() {
        // Page 1
        assert_eq!(0, Paging::new(1, 10).offset());
        // Page 2
        assert_eq!(10, Paging::new(2, 10).offset());
        // Custom limit
        assert_eq!(25, Paging::new(2, 25).offset());
        // None limit defaults to 1 for offset calculation
        assert_eq!(1, Paging::new(2, None).offset());
        assert_eq!(2, Paging::new(3, None).offset());
    }
}
