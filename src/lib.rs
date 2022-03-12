// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! # Remote Two API Models
//!
//! This crate provides structs and enums for the [Remote Two](https://www.yio-remote.com/) API
//! messages and data structures.
//!
//! It can be used for user based Rust integration drivers as the Home-Assistant integration.
//! _TODO_ add GitHub link once published!
//! The model is also being used in the remote-core implementation to make sure it is up-to-date
//! with the API specifications.
//!
//! ## API Specifications
//!
//! _TODO add links once published on GitHub_
//!
//! WebSocket APIs:
//! - Integration AsyncAPI
//! - Core AsyncAPI
//!
//! REST API:
//! - Core OpenAPI
//!

// Note: unfortunately the validator crate doesn't allow to use variables or constants for repeating
// message texts: <https://github.com/Keats/validator/issues/142>. Therefore the text length
// messages are duplicated all over...

#![forbid(non_ascii_idents)]
#![deny(unsafe_code)]

#[macro_use]
extern crate validator_derive;

pub use entity::*;
pub use integration::*;

mod entity;
mod integration;
pub mod web;
pub mod ws;
