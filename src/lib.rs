// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! # Remote Two/3 API Models
//!
//! This crate provides structs and enums for the [Remote Two/3 APIs](https://github.com/unfoldedcircle/core-api).
//!
//! The models can be used for user-based Rust integration drivers as the
//! [Home-Assistant integration](https://github.com/unfoldedcircle/integration-home-assistant).
//!
//! The model is also being used in the remote-core and simulator implementation to make sure it is
//! up-to-date with the API specifications.
//!
//! Notes:
//! - The models are manually defined and not auto-generated from the AsyncAPI & OpenAPI definitions.
//! - The defined structs are as simple as possible in terms of lifetimes and not optimized for
//!   Serde zero-copy deserialization. More information: <https://serde.rs/lifetimes.html>
//!
//! ## API Specifications
//!
//! See [core-api](https://github.com/unfoldedcircle/core-api) GitHub repository.
//!
//! WebSocket APIs:
//! - [Integration AsyncAPI](https://github.com/unfoldedcircle/core-api/tree/main/integration-api)
//! - [Core AsyncAPI](https://github.com/unfoldedcircle/core-api/tree/main/core-api)
//!
//! REST API:
//! - [Core OpenAPI](https://github.com/unfoldedcircle/core-api/tree/main/core-api)
//!

// Note: unfortunately, the validator crate doesn't allow using variables or constants for repeating
// message texts: <https://github.com/Keats/validator/issues/142>. Therefore, the text length
// messages are duplicated all over...

#![forbid(non_ascii_idents)]
#![deny(unsafe_code)]

#[macro_use]
extern crate validator_derive;

use once_cell::sync::Lazy;
use regex::Regex;

pub mod core;
mod entity;
pub mod intg;
pub mod model;
pub mod util;
pub mod ws;

pub use entity::*;

static REGEX_ID_CHARS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9-_]{1,}$").expect("invalid REGEX_ID_CHARS"));
static REGEX_ICON_ID: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9-_\\.:]{1,}$").expect("invalid REGEX_ICON_ID"));
static REGEX_LANGUAGE_CODE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z]{2}(_[A-Z]{2})?$").expect("invalid REGEX_LANGUAGE_CODE"));
/// HTTP header field name: RFC 9110 token characters.
static REGEX_HTTP_HEADER_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[!#$%&'*+.^_`|~0-9A-Za-z-]+$").expect("invalid REGEX_HTTP_HEADER_NAME")
});
/// Web view background color `#RRGGBB`. Empty: the default white `#FFFFFF`.
static REGEX_WEBVIEW_BACKGROUND_COLOR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(#[0-9a-fA-F]{6})?$").expect("invalid REGEX_WEBVIEW_BACKGROUND_COLOR")
});
/// SHA-256 certificate fingerprint: 64 hexadecimal digits, optionally separated by `:`.
static REGEX_WEBVIEW_CERTIFICATE_SHA256: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9a-fA-F]{2}(:?[0-9a-fA-F]{2}){31}$")
        .expect("invalid REGEX_WEBVIEW_CERTIFICATE_SHA256")
});
