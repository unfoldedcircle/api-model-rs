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

#[derive(Debug, Deserialize, Serialize, Validate)]
#[skip_serializing_none]
pub struct IntegrationDriver {
    #[validate(length(
        min = 1,
        max = 50,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 50)"
    ))]
    pub driver_id: String,
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub friendly_name: String,
    pub driver_url: String,
    pub auth_method: Option<String>,
    pub version: String,
    pub min_core_api: Option<String>,
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub icon: Option<String>,
    pub description: Option<String>,
    pub developer: Option<DriverDeveloper>,
    pub home_page: Option<String>,
    pub device_discovery: Option<bool>,
    pub setup_data_schema: Option<serde_json::Map<String, Value>>,
    pub release_date: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[skip_serializing_none]
pub struct DriverDeveloper {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
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
