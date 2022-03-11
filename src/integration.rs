// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! Integration related data structures.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

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

/// Minimal integration driver information.
///
/// This data structure is intended for driver overview pages.
#[derive(Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct IntegrationDriverInfo {
    pub driver_id: String,
    pub friendly_name: String,
    pub driver_url: String,
    pub version: String,
    pub icon: Option<String>,
}

/// Integration driver model.
#[derive(Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct IntegrationDriver {
    pub driver_id: String,
    pub friendly_name: String,
    pub driver_url: String,
    pub auth_method: Option<String>,
    pub version: String,
    pub min_core_api: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub developer: Option<DriverDeveloper>,
    pub home_page: Option<String>,
    pub device_discovery: bool,
    pub setup_data_schema: Option<serde_json::Map<String, Value>>,
    // TODO use chrono crate?!
    pub release_date: String,
}

/// Integration driver update model.
///
/// This is a dedicated model related to [`IntegrationDriver`] for create and patch update
/// operations with field validations.
/// The create operation will check required fields in the original model.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[skip_serializing_none]
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
    #[validate(length(max = 2048, message = "Invalid length (max = 2048)"))]
    pub driver_url: Option<String>,
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
    pub developer: Option<DriverDeveloper>,
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub home_page: Option<String>,
    pub device_discovery: Option<bool>,
    pub setup_data_schema: Option<serde_json::Map<String, Value>>,
    // TODO use chrono crate?!
    pub release_date: Option<String>,
}

/// Developer information for an integration driver.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[skip_serializing_none]
pub struct DriverDeveloper {
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub name: Option<String>,
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub url: Option<String>,
    #[validate(length(max = 100, message = "Invalid length (max = 100)"))]
    pub email: Option<String>,
}

/// Integration instance model.
#[derive(Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct Integration {
    pub integration_id: String,
    pub driver_id: String,
    pub friendly_name: String,
    pub icon: Option<String>,
    pub enabled: bool,
}

/// Integration instance update model.
///
/// This is a dedicated model related to [`Integration`] for create and patch update
/// operations with field validations.
/// The create operation will check required fields in the original model.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[skip_serializing_none]
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
}

/// Integration device states.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceState {
    Connecting,
    Connected,
    Disconnected,
    Error,
}
