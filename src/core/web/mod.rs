// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Core REST API specific messages.

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Rest API response
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct ApiResponse<'a> {
    pub code: Option<&'a str>,
    pub message: Option<&'a str>,
}

impl<'a> ApiResponse<'a> {
    pub fn new(code: &'a str, message: &'a str) -> ApiResponse<'a> {
        ApiResponse {
            code: Some(code),
            message: Some(message),
        }
    }
}
