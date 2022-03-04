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

pub use integration::*;

mod integration;
pub mod ws;
pub mod web;
