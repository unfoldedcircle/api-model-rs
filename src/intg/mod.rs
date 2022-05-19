// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration API related data structures, independent of the transport layer.
//!
//! See `ws` sub module for WebSocket specific message structures.

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
#[cfg(feature = "sqlx")]
use sqlx::types::Json;
use validator::Validate;

use crate::ws::WsAuthentication;
use crate::{REGEX_ICON_ID, REGEX_ID_CHARS};

mod entity;
pub mod ws;

pub use entity::*;

/// Integration driver version information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationVersion {
    /// Implemented API version.
    pub api: String,
    /// Version of the integration.
    pub integration: String,
}

/// Subscribe to events.
///
/// Subscribe to entity state change events to receive `entity_change` events from the integration
/// driver.  
/// If no entity IDs are specified then events for all available entities are sent to the remote.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationStatus {
    /// Integration instance identifier.  
    pub integration_id: String,
    /// Name of the integration driver.  
    /// Key value pairs of language texts. Key: ISO 639-1 code with optional country suffix.
    pub name: HashMap<String, String>,
    /// Optional icon identifier of the integration.
    pub icon: Option<String>,
    /// Device state. This is the last known state of the device.
    pub state: DeviceState,
    /// Integration is enabled
    pub enabled: bool,
}

/// Minimal integration driver information.
///
/// This data structure is intended for driver overview pages.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationDriverInfo {
    /// Integration driver identifier.  
    pub driver_id: String,
    /// Name of the driver.  
    /// Key value pairs of language texts. Key: ISO 639-1 code with optional country suffix.
    pub name: HashMap<String, String>,
    pub driver_url: String,
    pub version: String,
    /// Optional icon identifier of the integration driver.
    pub icon: Option<String>,
    pub enabled: bool,
}

/// Integration driver model.
///
/// A driver represents the communication aspect of an integration. E.g. how one can connect to it
/// and which API version it supports.
///
/// One driver can provide multiple [`Integration`] instances. In the integration API they are
/// referred to as `multi-device integrations` and use the optional `device_id` property where
/// required. If a driver only provides a single instance, which is usually the default use case,
/// then the `device_id` is not used (or set to the default value `main`).
///
/// **Attention:** For now this is just a 1:1 relationship until the multi-device setup is finalized
/// in the API!
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationDriver {
    /// Unique driver identifier.  
    /// Provided by the user or during driver registration. Otherwise a generated UUID.
    pub driver_id: String,
    /// Name of the driver to display in the UI.  
    /// Key value pairs of language texts. Key: ISO 639-1 code with optional country suffix to
    /// represent a `culture code`. Examples: `en`, `en-UK`, `en-US`, `de`, `de-CH`.  
    /// An english text with key `en` should always be provided as fallback option. Otherwise it's
    /// not guaranteed which text will be displayed if the user selected language is not provided.
    pub name: HashMap<String, String>,
    /// WebSocket URL of the integration driver.
    pub driver_url: String,
    /// Optional driver authentication token.
    ///
    /// Note: the token will not be returned to external clients!
    pub token: Option<String>,
    /// Authentication method if token is used.
    pub auth_method: Option<WsAuthentication>,
    /// Driver version, [SemVer](https://semver.org/) preferred.
    pub version: String,
    /// Optional version check: minimum required core API version in the remote.
    pub min_core_api: Option<String>,
    /// Optional icon identifier of the integration driver.
    pub icon: Option<String>,
    /// Enables or disables driver communication.
    /// If disabled, all integration instances won't be activated, even if the instance is enabled.
    pub enabled: bool,
    /// Optional description of the integration.  
    /// Key value pairs of language texts.
    pub description: Option<HashMap<String, String>>,
    /// Optional information about the integration developer or company.
    pub developer: Option<DriverDeveloper>,
    /// Optional home page url for more information.
    pub home_page: Option<String>,
    /// Driver supports multi-device discovery. **Not yet supported**.
    pub device_discovery: bool,
    /// Driver configuration metadata describing configuration parameters for the web-configurator.
    /// **Not yet finalized**.
    #[cfg(feature = "sqlx")]
    pub setup_data_schema: Json<Value>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data_schema: Value,
    /// Release date of the driver.
    pub release_date: Option<NaiveDate>,
}

/// Integration driver update model.
///
/// This is a dedicated model related to [`IntegrationDriver`] for create and patch update
/// operations with field validations.
/// The create operation will check required fields in the original model.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct IntegrationDriverUpdate {
    /// Integration driver identifier.  
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    #[validate(regex(path = "REGEX_ID_CHARS"))]
    pub driver_id: Option<String>,
    // TODO validate HashMap with custom validation function?
    pub name: Option<HashMap<String, String>>,
    #[validate(url)]
    #[validate(length(max = 2048, message = "Invalid length (max = 2048)"))]
    pub driver_url: Option<String>,
    #[validate(length(max = 2048, message = "Invalid length (max = 2048)"))]
    pub token: Option<String>,
    pub auth_method: Option<WsAuthentication>,
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub version: Option<String>,
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub min_core_api: Option<String>,
    /// Optional icon identifier of the integration driver.
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub icon: Option<String>,
    pub enabled: Option<bool>,
    pub description: Option<HashMap<String, String>>,
    #[validate]
    pub developer: Option<DriverDeveloper>,
    #[validate(url)]
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub home_page: Option<String>,
    pub device_discovery: Option<bool>,
    #[cfg(feature = "sqlx")]
    pub setup_data_schema: Option<Json<Value>>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data_schema: Option<Value>,
    pub release_date: Option<NaiveDate>,
}

/// Developer information for an integration driver.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct DriverDeveloper {
    #[validate(length(max = 100, message = "Invalid length (max = 100)"))]
    pub name: Option<String>,
    #[validate(url)]
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub url: Option<String>,
    #[validate(email)]
    #[validate(length(max = 100, message = "Invalid length (max = 100)"))]
    pub email: Option<String>,
}

impl From<IntegrationDriver> for IntegrationDriverUpdate {
    fn from(drv: IntegrationDriver) -> Self {
        Self {
            driver_id: Some(drv.driver_id),
            name: Some(drv.name),
            driver_url: Some(drv.driver_url),
            token: drv.token,
            auth_method: drv.auth_method,
            version: Some(drv.version),
            min_core_api: drv.min_core_api,
            icon: drv.icon,
            enabled: Some(drv.enabled),
            description: drv.description,
            developer: drv.developer,
            home_page: drv.home_page,
            device_discovery: Some(drv.device_discovery),
            setup_data_schema: Some(drv.setup_data_schema),
            release_date: drv.release_date,
        }
    }
}

/// Integration instance model.
///
/// An integration instance represents a configured integration driver.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Integration {
    /// Unique integration identifier.  
    /// Provided by the user or during driver registration. Otherwise a generated UUID.
    pub integration_id: String,
    /// The required integration driver. References ['IntegrationDriver'].
    pub driver_id: String,
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    /// Name of the integration instance.  
    /// Usually the default driver name and an optional device identifier.  
    /// Key value pairs of language texts. Key: ISO 639-1 code with optional country suffix.
    pub name: HashMap<String, String>,
    /// Optional icon identifier of the integration.
    pub icon: Option<String>,
    pub enabled: bool,
    /// Optional configuration data if supported or required by the driver.
    #[cfg(feature = "sqlx")]
    pub setup_data: Json<serde_json::Map<String, Value>>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data: serde_json::Map<String, Value>,
}

/// Integration instance update model.
///
/// This is a dedicated model related to [`Integration`] for create and patch update
/// operations with field validations.
/// The create operation will check required fields in the original model.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct IntegrationUpdate {
    /// Unique integration instance identifier. ID is set by the system.
    /// This field cannot be updated
    pub integration_id: Option<String>,
    /// The required integration driver. References ['IntegrationDriver'].
    /// This field cannot be updated
    pub driver_id: Option<String>,
    /// Only required for multi-device integrations.
    /// This field cannot be updated.
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    #[validate(regex(path = "REGEX_ID_CHARS", code = "INVALID_CHARACTERS"))]
    pub device_id: Option<String>,
    pub name: Option<HashMap<String, String>>,
    /// Optional icon identifier of the integration.
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    #[validate(regex(path = "REGEX_ICON_ID", code = "INVALID_CHARACTERS"))]
    pub icon: Option<String>,
    pub enabled: Option<bool>,
    #[cfg(feature = "sqlx")]
    pub setup_data: Option<Json<serde_json::Map<String, Value>>>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data: Option<serde_json::Map<String, Value>>,
}

impl From<Integration> for IntegrationUpdate {
    fn from(intg: Integration) -> Self {
        Self {
            integration_id: Some(intg.integration_id),
            driver_id: Some(intg.driver_id),
            device_id: intg.device_id,
            name: Some(intg.name),
            icon: intg.icon,
            enabled: Some(intg.enabled),
            setup_data: Some(intg.setup_data),
        }
    }
}

/// Integration device states.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
#[derive(
    Debug, Clone, strum_macros::Display, strum_macros::EnumString, PartialEq, Deserialize, Serialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum DeviceState {
    Connecting,
    Connected,
    Disconnected,
    Error,
}
