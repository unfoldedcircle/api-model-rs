// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! Integration related data structures.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
#[cfg(feature = "sqlx")]
use sqlx::types::Json;
use validator::Validate;

/// Integration driver version information.
#[derive(Debug, Serialize)]
pub struct IntegrationVersion {
    /// Implemented API version.
    pub api: String,
    /// Version of the integration.
    pub integration: String,
}

/// Subscribe to events.
///
/// Subscribe to entity state change events to receive `entity_change` events from the integration driver.
/// If no entity IDs are specified then events for all available entities are sent to the remote.
#[derive(Debug, Deserialize)]
pub struct SubscribeEvents {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    /// Subscribe to events for the specified entities.
    pub entity_ids: Vec<String>,
}

/// Integration status information.
///
/// Provides integration instance information.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegrationStatus {
    pub integration_id: String,
    pub friendly_name: String,
    pub icon: Option<String>,
    //pub state: DeviceState,
    pub enabled: bool,
}

/// Minimal integration driver information.
///
/// This data structure is intended for driver overview pages.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegrationDriverInfo {
    pub driver_id: String,
    pub friendly_name: String,
    pub driver_url: String,
    pub version: String,
    pub icon: Option<String>,
}

/// Integration driver model.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegrationDriver {
    /// Unique driver identifier. Either provided by the user, otherwise a generated UUID.
    pub driver_id: String,
    /// Name of the driver to display in the UI.
    pub friendly_name: String,
    /// WebSocket URL of the integration driver.
    pub driver_url: String,
    /// Optional driver authentication token.
    ///
    /// Note: the token will not be returned to external clients!
    pub token: Option<String>,
    /// Authentication method if token is used.
    pub auth_method: Option<String>,
    /// Driver version, [SemVer](https://semver.org/) preferred.
    pub version: String,
    /// Optional version check: minimum required core API version in the remote.
    pub min_core_api: Option<String>,
    pub icon: Option<String>,
    /// Optional description of the integration.
    pub description: Option<String>,
    /// Optional information about the integration developer or company.
    pub developer: Option<DriverDeveloper>,
    /// Optional home page url for more information.
    pub home_page: Option<String>,
    /// Driver supports multi device discovery.
    pub device_discovery: bool,
    #[cfg(feature = "sqlx")]
    pub setup_data_schema: Json<serde_json::Value>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data_schema: Option<serde_json::Value>,
    /// Release date of the driver.
    pub release_date: Option<NaiveDate>,
}

/// Integration driver update model.
///
/// This is a dedicated model related to [`IntegrationDriver`] for create and patch update
/// operations with field validations.
/// The create operation will check required fields in the original model.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct IntegrationDriverUpdate {
    #[validate(length(
        min = 1,
        max = 36,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 50)"
    ))]
    pub driver_id: Option<String>,
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub friendly_name: Option<String>,
    #[validate(url)]
    #[validate(length(max = 2048, message = "Invalid length (max = 2048)"))]
    pub driver_url: Option<String>,
    #[validate(length(max = 2048, message = "Invalid length (max = 2048)"))]
    pub token: Option<String>,
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub auth_method: Option<String>,
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub version: Option<String>,
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub min_core_api: Option<String>,
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub icon: Option<String>,
    #[validate(length(max = 2048, message = "Invalid length (max = 2048)"))]
    pub description: Option<String>,
    #[validate]
    pub developer: Option<DriverDeveloper>,
    #[validate(url)]
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub home_page: Option<String>,
    pub device_discovery: Option<bool>,
    #[cfg(feature = "sqlx")]
    pub setup_data_schema: Option<Json<serde_json::Value>>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data_schema: Option<serde_json::Value>,
    pub release_date: Option<NaiveDate>,
}

/// Developer information for an integration driver.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DriverDeveloper {
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub name: Option<String>,
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub url: Option<String>,
    #[validate(length(max = 100, message = "Invalid length (max = 100)"))]
    pub email: Option<String>,
}

/// Integration instance update model.
///
/// This is a dedicated model related to [`Integration`] for create and patch update
/// operations with field validations.
/// The create operation will check required fields in the original model.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct IntegrationUpdate {
    #[validate(length(
        min = 1,
        max = 36,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 36)"
    ))]
    pub integration_id: Option<String>,
    #[validate(length(
        min = 1,
        max = 36,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 50)"
    ))]
    pub driver_id: Option<String>,
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub friendly_name: Option<String>,
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub icon: Option<String>,
    pub enabled: Option<bool>,
    #[cfg(feature = "sqlx")]
    pub setup_data: Option<Json<serde_json::Map<String, Value>>>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data: Option<serde_json::Map<String, Value>>,
}

/// Integration instance model.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Integration {
    pub integration_id: String,
    pub driver_id: String,
    pub friendly_name: String,
    pub icon: Option<String>,
    pub enabled: bool,
    #[cfg(feature = "sqlx")]
    pub setup_data: Json<serde_json::Map<String, Value>>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data: Option<serde_json::Map<String, Value>>,
}

/// Integration device states.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
// TODO do we need numeric representation?
#[derive(
    Debug, strum_macros::Display, strum_macros::EnumString, PartialEq, Serialize, Deserialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceState {
    Connecting,
    Connected,
    Disconnected,
    Error,
}
