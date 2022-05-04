// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! # Remote Two API Models
//!
//! This crate provides structs and enums for the [Remote Two APIs](https://github.com/unfoldedcircle/core-api).
//!
//! It can be used for user based Rust integration drivers as the Home-Assistant integration.
//! _TODO_ add GitHub link once published!
//!
//! The model is also being used in the remote-core implementation to make sure it is up-to-date
//! with the API specifications.
//!
//! Note: the defined structs are as simple as possible in terms of lifetimes and not optimized for
//! Serde zero-copy deserialization. More information: <https://serde.rs/lifetimes.html>
//!
//! ## API Specifications
//!
//! See [core-api](https://github.com/unfoldedcircle/core-api) GitHub repository.
//!
//! WebSocket APIs:
//! - [Integration AsyncAPI](https://github.com/unfoldedcircle/core-api/tree/main/integration-api)
//! - Core AsyncAPI - _not yet published_
//!
//! REST API:
//! - Core OpenAPI - _not yet published_
//!

// Note: unfortunately the validator crate doesn't allow to use variables or constants for repeating
// message texts: <https://github.com/Keats/validator/issues/142>. Therefore the text length
// messages are duplicated all over...

#![forbid(non_ascii_idents)]
#![deny(unsafe_code)]

#[macro_use]
extern crate validator_derive;

use lazy_static::lazy_static;
use regex::Regex;

pub mod core;
mod entity;
pub mod intg;
pub mod ws;

pub use entity::*;

lazy_static! {
    static ref REGEX_ID_CHARS: Regex = Regex::new(r"^[a-zA-Z0-9-_]{1,}$").unwrap(); // max length is a dedicated validation for better error messages
    static ref REGEX_ICON_ID: Regex = Regex::new(r"^[a-zA-Z0-9-_\\.:]{1,}$").unwrap();
}
