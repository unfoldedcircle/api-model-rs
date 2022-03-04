// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! REST API specific messages.

use serde::{Serialize};

/// Rest API response
#[derive(Debug, Serialize)]
pub struct ApiResponse<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
}
