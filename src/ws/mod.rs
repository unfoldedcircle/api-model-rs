// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Common WebSocket messages used for Core & Integration APIs.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use strum_macros::*;

/// WebSocket authentication type.
#[derive(
    Debug, Clone, Copy, AsRefStr, Display, EnumString, PartialEq, Eq, Serialize, Deserialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
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
///
/// # Examples
///
/// Deserialize from JSON:
/// ```
/// use uc_api::ws::WsMessage;
/// let json = serde_json::json!({
///     "kind": "req",
///     "id": 123,
///     "msg": "test",
///     "msg_data": {
///         "foo": "bar"
///     },
///     "bar": "foo"
/// });
/// let request: WsMessage = serde_json::from_value(json).expect("Invalid json message");
/// assert_eq!(Some("req"), request.kind.as_deref());
/// assert_eq!(Some(123), request.id);
/// assert_eq!(None, request.req_id);
/// assert_eq!(Some("test"), request.msg.as_deref());
/// assert_eq!(None, request.code);
/// assert_eq!(None, request.cat);
/// assert_eq!(None, request.ts);
/// assert!(request.msg_data.is_some());
/// let msg_data = request.msg_data.unwrap_or_default();
/// assert_eq!(Some("bar"), msg_data.get("foo").and_then(|v| v.as_str()));
/// assert!(request.extra.contains_key("bar"));
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    /// Helper method to create a `WsMessage` struct representing an event message.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use chrono::{DateTime, Utc};
    /// use serde_json::json;
    /// use uc_api::ws::{EventCategory, WsMessage, WsResultMsgData};
    ///
    /// let now = Utc::now();
    /// let event = WsMessage::event("test_event", EventCategory::Device, json!({ "foo": "bar" }));
    /// let json = serde_json::to_value(event).unwrap();
    /// let ts = json.as_object().and_then(|o| o.get("ts")).and_then(|v| v.as_str()).expect("Missing ts property");
    /// let ts_utc: DateTime<Utc> = ts.parse().expect("invalid ts format");
    /// assert!(ts_utc >= now);
    /// let expected = json!(
    ///   {
    ///     "kind": "event",
    ///     "msg": "test_event",
    ///     "cat": "DEVICE",
    ///     "msg_data": {
    ///         "foo": "bar",
    ///     },
    ///     "ts": ts
    ///   }
    /// );
    /// assert_eq!(expected, json);
    /// ```
    pub fn event(
        msg: impl Into<String>,
        cat: impl Into<Option<EventCategory>>,
        msg_data: Value,
    ) -> Self {
        Self {
            kind: Some("event".into()),
            msg: Some(msg.into()),
            cat: cat.into(),
            ts: Some(Utc::now()),
            msg_data: Some(msg_data),
            ..Default::default()
        }
    }

    /// Helper method to create `WsMessage` struct representing a request message without `msg_data`.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsMessage, WsResultMsgData};
    /// let response = WsMessage::simple_request(123, "test_request");
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "req",
    ///     "id": 123,
    ///     "msg": "test_request"
    /// }), json);
    ///
    /// ```
    pub fn simple_request(id: u32, msg: impl Into<String>) -> Self {
        Self {
            kind: Some("req".into()),
            id: Some(id),
            msg: Some(msg.into()),
            ..Default::default()
        }
    }

    /// Helper method to create `WsMessage` struct representing a request message.
    ///
    /// The `msg_data` property will be set from the provided serializable struct.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsMessage, WsResultMsgData};
    /// let response = WsMessage::request(123, "test_request", WsResultMsgData::new("42", "testing")).expect("Invalid msg_data");
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "req",
    ///     "id": 123,
    ///     "msg": "test_request",
    ///     "msg_data": {
    ///         "code": "42",
    ///         "message": "testing"
    ///     }
    /// }), json);
    ///
    /// ```
    pub fn request<T: serde::Serialize>(
        id: u32,
        msg: impl Into<String>,
        msg_data: T,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            kind: Some("req".into()),
            id: Some(id),
            msg: Some(msg.into()),
            msg_data: Some(serde_json::to_value(msg_data)?),
            ..Default::default()
        })
    }

    /// Helper method to create a `WsMessage` struct representing a successful response message.
    ///
    /// The `msg_data` property will be set from the provided json value and the response `code`
    /// will be set to `200`.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use serde_json::json;
    /// use uc_api::ws::WsMessage;
    /// let response = WsMessage::response_json(123, "test_resp", json!({ "foo": "bar" }));
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "test_resp",
    ///     "code": 200,
    ///     "msg_data": {
    ///         "foo": "bar"
    ///     }
    /// }), json);
    ///
    /// ```
    pub fn response_json(req_id: u32, msg: impl Into<String>, msg_data: Value) -> Self {
        Self {
            kind: Some("resp".into()),
            req_id: Some(req_id),
            msg: Some(msg.into()),
            code: Some(200),
            msg_data: Some(msg_data),
            ..Default::default()
        }
    }

    /// Helper method to create `WsMessage` struct representing a successful response message.
    ///
    /// The `msg_data` property will be set from the provided serializable struct and the response
    /// `code` will be set to `200`.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsMessage, WsResultMsgData};
    /// let response = WsMessage::response(123, "test_resp", WsResultMsgData::new("OK", "success"));
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "test_resp",
    ///     "code": 200,
    ///     "msg_data": {
    ///         "code": "OK",
    ///         "message": "success"
    ///     }
    /// }), json);
    ///
    /// ```
    pub fn response<T: serde::Serialize>(req_id: u32, msg: impl Into<String>, msg_data: T) -> Self {
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

    /// Helper method to create a `WsMessage` struct representing an error response message.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsMessage, WsResultMsgData};
    /// let response = WsMessage::error(123, 400, WsResultMsgData::new("ERROR", "foobar"));
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "result",
    ///     "code": 400,
    ///     "msg_data": {
    ///         "code": "ERROR",
    ///         "message": "foobar"
    ///     }
    /// }), json);
    ///
    /// ```
    pub fn error(req_id: u32, code: u16, msg_data: WsResultMsgData) -> Self {
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
///
/// # Examples
///
/// Deserialize from JSON:
/// ```
/// use uc_api::ws::WsRequest;
/// let json = serde_json::json!({
///     "kind": "req",
///     "id": 123,
///     "msg": "test",
///     "msg_data": {
///         "foo": "bar"
///     }   
/// });
/// let request: WsRequest = serde_json::from_value(json).expect("Invalid json message");
/// assert_eq!("req", &request.kind);
/// assert_eq!(123, request.id);
/// assert_eq!("test", &request.msg);
/// let msg_data = request.msg_data.unwrap_or_default();
/// assert_eq!(Some("bar"), msg_data.get("foo").and_then(|v| v.as_str()));
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl WsRequest {
    /// Helper method to create a request message from a serializable struct as msg_data payload.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use serde_json::json;
    /// use uc_api::ws::{WsRequest, WsResultMsgData};
    /// let request = WsRequest::new(123, "test_request", WsResultMsgData::new("OK", "success"));
    /// let json = serde_json::to_value(request.unwrap()).unwrap();
    /// assert_eq!(json!({
    ///     "kind": "req",
    ///     "id": 123,
    ///     "msg": "test_request",
    ///     "msg_data": {
    ///         "code": "OK",
    ///         "message": "success"
    ///     }
    /// }), json);
    /// ```
    pub fn new<T: serde::Serialize>(
        id: u32,
        msg: impl Into<String>,
        msg_data: T,
    ) -> Result<Self, serde_json::Error> {
        let msg_data = serde_json::to_value(msg_data)?;
        Ok(Self {
            kind: "req".into(),
            id,
            msg: msg.into(),
            msg_data: Some(msg_data),
        })
    }
}

impl From<WsRequest> for WsMessage {
    fn from(r: WsRequest) -> Self {
        Self {
            kind: Some(r.kind),
            id: Some(r.id),
            msg: Some(r.msg),
            msg_data: r.msg_data,
            ..Default::default()
        }
    }
}

/// Common response message.
///
/// # Examples
///
/// Serialize to JSON:
/// ```
/// let response = uc_api::ws::WsResponse {
///     kind: "resp".to_string(),
///     req_id: 123,
///     msg: "test_result".to_string(),
///     code: 200,
///     msg_data: None,
/// };
/// let json = serde_json::to_value(response).unwrap();
/// assert_eq!(serde_json::json!({
///     "kind": "resp",
///     "req_id": 123,
///     "msg": "test_result",
///     "code": 200
/// }), json);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    /// Helper method to create a response message from a serializable struct as msg_data payload.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use serde_json::json;
    /// use uc_api::ws::{WsResponse, WsResultMsgData};
    /// let response = WsResponse::new(123, "test_result", WsResultMsgData::new("OK", "success"));
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "test_result",
    ///     "code": 200,
    ///     "msg_data": {
    ///         "code": "OK",
    ///         "message": "success"
    ///     }
    /// }), json);
    /// ```
    pub fn new<T: serde::Serialize>(req_id: u32, msg: impl Into<String>, msg_data: T) -> Self {
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

    /// Helper method to create an error response message.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsResponse, WsResultMsgData};
    /// let response = WsResponse::error(123, 400, WsResultMsgData::new("ERROR", "foobar"));
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "result",
    ///     "code": 400,
    ///     "msg_data": {
    ///         "code": "ERROR",
    ///         "message": "foobar"
    ///     }
    /// }), json);
    ///
    /// ```
    pub fn error(req_id: u32, code: u16, msg_data: WsResultMsgData) -> Self {
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

    /// Helper method to create a 400 "bad request" error response message for a missing field.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsResponse, WsResultMsgData};
    /// let response = WsResponse::missing_field(123, "foobar");
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "result",
    ///     "code": 400,
    ///     "msg_data": {
    ///         "code": "BAD_REQUEST",
    ///         "message": "Missing field: foobar"
    ///     }
    /// }), json);
    ///
    /// ```
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

    /// Helper method to create a 404 "not found" error response message with a custom message.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsResponse, WsResultMsgData};
    /// let response = WsResponse::not_found(123, "custom error text");
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "result",
    ///     "code": 404,
    ///     "msg_data": {
    ///         "code": "NOT_FOUND",
    ///         "message": "custom error text"
    ///     }
    /// }), json);
    ///
    /// ```
    pub fn not_found(req_id: u32, message: impl Into<String>) -> Self {
        Self {
            kind: "resp".into(),
            req_id,
            msg: "result".into(),
            code: 404,
            msg_data: Some(json!({ "code": "NOT_FOUND", "message": message.into() })),
        }
    }

    /// Helper method to create a simple response message without `msg_data` payload.
    ///
    /// # Examples
    ///
    /// Serialize to JSON:
    /// ```
    /// use uc_api::ws::{WsResponse, WsResultMsgData};
    /// let response = WsResponse::result(123, 201);
    /// let json = serde_json::to_value(response).unwrap();
    /// assert_eq!(serde_json::json!({
    ///     "kind": "resp",
    ///     "req_id": 123,
    ///     "msg": "result",
    ///     "code": 201
    /// }), json);
    ///
    /// ```
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

impl From<WsResponse> for WsMessage {
    fn from(r: WsResponse) -> Self {
        Self {
            kind: Some(r.kind),
            req_id: Some(r.req_id),
            msg: Some(r.msg),
            code: Some(r.code),
            msg_data: r.msg_data,
            ..Default::default()
        }
    }
}

/// Default payload data of `result` response message in `msg_data` property.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WsResultMsgData {
    pub code: String,
    pub message: String,
}

impl WsResultMsgData {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Event message categories.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventCategory {
    /// Device specific events like integration driver status changes
    Device,
    /// Entity change events
    Entity,
    /// Remote specific events like configuration changes
    Remote,
    /// UI change events
    Ui,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_to_message_conversion() {
        let request = WsRequest::new(123, "test_request", WsResultMsgData::new("OK", "testing"))
            .expect("bug: may not fail");
        let msg = WsMessage::from(request);

        let json = serde_json::to_value(msg).unwrap();
        assert_eq!(
            serde_json::json!({
                "kind": "req",
                "id": 123,
                "msg": "test_request",
                "msg_data": {
                    "code": "OK",
                    "message": "testing"
                }
            }),
            json
        );
    }

    #[test]
    fn response_to_message_conversion() {
        let response = WsResponse::result(123, 201);
        let msg = WsMessage::from(response);

        let json = serde_json::to_value(msg).unwrap();
        assert_eq!(
            serde_json::json!({
                "kind": "resp",
                "req_id": 123,
                "msg": "result",
                "code": 201
            }),
            json
        );
    }
}
