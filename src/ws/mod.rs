// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! WebSocket specific messages.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

pub mod intg;

/// WebSocket authentication type.
#[derive(
    Debug, Clone, strum_macros::Display, strum_macros::EnumString, PartialEq, Serialize, Deserialize,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum WsAuthentication {
    /// Authenticate with header token.
    Header,
    /// Authenticate with authentication message.
    Message,
}

/// Generic message definition for requests, responses and events.
///
/// This message structure is for best effort parsing. See [`WsRequest`] and [`WsResponse`] for
/// specific message definitions.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WsMessage {
    /// Message identifier: `req`, `resp`, `event`
    pub kind: Option<String>,
    /// Request message only: ID which must be increased for every new request. This ID will be returned in the response message.
    pub id: Option<u32>,
    /// Response message only: corresponding request ID.
    pub req_id: Option<u32>,
    /// One of the defined API message types.
    pub msg: Option<String>,
    /// Response message only: code of the operation according to HTTP status codes.
    pub code: Option<u16>,
    /// Event message only: category of the event.
    pub cat: Option<EventCategory>,
    /// Event message only: optional timestamp when the event was generated.
    pub ts: Option<DateTime<Utc>>,
    /// Message payload.
    pub msg_data: Option<Value>,
    /// Extra fields
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl WsMessage {
    pub fn event(msg: &str, cat: Option<EventCategory>, msg_data: Value) -> Self {
        Self {
            kind: Some("event".into()),
            msg: Some(msg.into()),
            cat,
            ts: Some(Utc::now()),
            msg_data: Some(msg_data),
            ..Default::default()
        }
    }

    pub fn response_json(req_id: u32, msg: &str, msg_data: Value) -> Self {
        Self {
            kind: Some("resp".into()),
            req_id: Some(req_id),
            msg: Some(msg.into()),
            msg_data: Some(msg_data),
            ..Default::default()
        }
    }

    pub fn response<T: serde::Serialize>(req_id: u32, msg: &str, msg_data: T) -> Self {
        match serde_json::to_value(msg_data) {
            Ok(v) => Self {
                kind: Some("resp".into()),
                req_id: Some(req_id),
                msg: Some(msg.into()),
                code: Some(200),
                msg_data: Some(v),
                ..Default::default()
            },

            Err(_) => Self {
                kind: Some("resp".into()),
                req_id: Some(req_id),
                msg: Some("result".into()),
                code: Some(500),
                msg_data: Some(
                    json!({ "code": "INTERNAL_ERROR", "message": "Error serializing result"}),
                ),
                ..Default::default()
            },
        }
    }

    pub fn error(req_id: u32, code: u16, msg_data: WsError) -> Self {
        Self {
            kind: Some("resp".into()),
            req_id: Some(req_id),
            msg: Some("result".into()),
            code: Some(code),
            msg_data: Some(
                serde_json::to_value(msg_data).expect("Error serializing model::Error struct"),
            ),
            ..Default::default()
        }
    }
}

/// Common request message.
#[derive(Debug, Deserialize, Serialize)]
pub struct WsRequest {
    /// Request message identifier: `req`
    pub kind: String,
    /// Request ID which must be increased for every new request.
    /// This ID will be returned in the response message.
    pub id: u32,
    /// One of the defined API request message types.
    pub msg: String,
    /// Message specific payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_data: Option<Value>,
}

/// Common response message.
#[derive(Debug, Deserialize, Serialize)]
pub struct WsResponse {
    /// Response message identifier: `resp`
    pub kind: String,
    /// Corresponding request ID.
    pub req_id: u32,
    /// One of the defined API response message types.
    pub msg: String,
    /// Response code of the operation according to HTTP status codes.
    pub code: u16,
    /// Message specific payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_data: Option<Value>,
}

impl WsResponse {
    pub fn new<T: serde::Serialize>(req_id: u32, msg: &str, msg_data: T) -> Self {
        // even though our structs should always be able to deserialize, better be safe...
        match serde_json::to_value(msg_data) {
            Ok(v) => Self {
                kind: "resp".into(),
                req_id,
                msg: msg.into(),
                code: 200,
                msg_data: Some(v),
            },
            Err(_) => Self {
                kind: "resp".into(),
                req_id,
                msg: "result".into(),
                code: 500,
                msg_data: Some(
                    json!({ "code": "INTERNAL_ERROR", "message": "Error serializing result"}),
                ),
            },
        }
    }

    pub fn error(req_id: u32, code: u16, msg_data: WsError) -> Self {
        Self {
            kind: "resp".into(),
            req_id,
            msg: "result".into(),
            code,
            msg_data: Some(
                serde_json::to_value(msg_data).expect("Error serializing WsError struct"),
            ),
        }
    }

    pub fn missing_field(req_id: u32, field: &str) -> Self {
        Self {
            kind: "resp".into(),
            req_id,
            msg: "result".into(),
            code: 400,
            msg_data: Some(
                json!({ "code": "BAD_REQUEST", "message": format!("Missing field: {}", field)}),
            ),
        }
    }

    pub fn result(req_id: u32, code: u16) -> Self {
        Self {
            kind: "resp".into(),
            req_id,
            msg: "result".into(),
            code,
            msg_data: None,
        }
    }
}

/// Error object.
#[derive(Debug, Serialize)]
pub struct WsError {
    pub code: String,
    pub message: String,
}

/// Event message categories.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventCategory {
    Device,
    Entity,
    Remote,
}
