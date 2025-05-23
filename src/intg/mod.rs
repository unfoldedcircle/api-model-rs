// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration API related data structures, independent of the transport layer.
//!
//! See `ws` sub module for WebSocket specific message structures.

mod entity;
pub mod ws;

pub use entity::*;

use crate::model::intg::{
    IntegrationSetupError, IntegrationSetupState, RequireUserAction, SetupChangeEventType,
};
use crate::ws::WsAuthentication;
use crate::{REGEX_ICON_ID, REGEX_ID_CHARS};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
#[cfg(feature = "sqlx")]
use sqlx::types::Json;
use std::collections::HashMap;
use strum_macros::*;
use validator::Validate;

/// Integration driver version information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationVersion {
    /// Implemented API version.
    pub api: Option<String>,
    /// Version of the integration.
    pub driver: Option<String>,
}

/// Subscribe to events.
///
/// Subscribe to entity state change events to receive `entity_change` events from the integration
/// driver.  
/// If no entity IDs are specified then events for all available entities are sent to the remote.
#[skip_serializing_none]
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
    /// Integration driver identifier.
    pub driver_id: Option<String>,
    /// Integration instance identifier.  
    pub integration_id: Option<String>,
    /// Name of the integration driver.  
    /// Key value pairs of language texts. Key: ISO 639-1 code with optional country suffix.
    pub name: HashMap<String, String>,
    /// Optional icon identifier of the integration.
    pub icon: Option<String>,
    pub driver_type: DriverType,
    /// Integration state.
    pub state: Option<IntegrationState>,
    /// Device state. This is the last known state of the device sent by the integration driver.
    #[deprecated(note = "Use state instead")]
    pub device_state: Option<DeviceState>,
    /// Integration driver connection state.
    #[deprecated(note = "Use state instead")]
    pub driver_state: Option<DriverState>,
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
    pub developer_name: Option<String>,
    pub driver_type: DriverType,
    pub driver_url: String,
    pub version: String,
    /// Optional icon identifier of the integration driver.
    pub icon: Option<String>,
    pub enabled: bool,
    /// true: multi-instance driver with device discovery, false: single instance driver.
    pub device_discovery: bool,
    /// Number of integration instances.
    pub instance_count: u16,
    /// Current state. `Idle` if the driver is not in use.
    pub driver_state: Option<DriverState>,
}

/// Message data payload of `setup_driver` to start driver setup.
///
/// If a driver includes a `setup_data_schema` object in its driver metadata, it
/// enables the dynamic driver setup process. The setup process can be a simple
/// "start-confirm-done" between the Remote Two and the integration driver, or a fully
/// dynamic, multistep process with user interactions, where the user has to provide
/// additional data or select different options.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetupDriver {
    /// Flag to distinguish regular driver setup vs. driver reconfiguration.
    pub reconfigure: Option<bool>,
    /// Input values of the initial setup page if it contains input fields and not just text.
    /// The key is the input field identifier, value contains the input value.
    pub setup_data: HashMap<String, String>,
}

/// Message data payload of `driver_setup_change`.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriverSetupChange {
    pub event_type: SetupChangeEventType,
    pub state: IntegrationSetupState,
    pub error: Option<IntegrationSetupError>,
    pub require_user_action: Option<RequireUserAction>,
}

/// Message data payload of `set_driver_user_data`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationSetup {
    /// User provided input values of a settings page.
    ///
    /// Key is the input field identifier, value the provided value in string format.
    InputValues(HashMap<String, String>),
    /// User confirmation.
    ///
    /// Attention: value is always true!
    /// If the user didn't confirm the setup settings page, the setup flow is aborted.
    Confirm(bool),
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
    /// Provided by the user or during driver registration. Otherwise, a generated UUID.
    pub driver_id: String,
    /// Name of the driver to display in the UI.  
    /// Key value pairs of language texts. Key: ISO 639-1 code with optional country suffix to
    /// represent a `culture code`. Examples: `en`, `en-UK`, `en-US`, `de`, `de-CH`.  
    /// An english text with key `en` should always be provided as fallback option. Otherwise, it's
    /// not guaranteed which text will be displayed if the user selected language is not provided.
    pub name: HashMap<String, String>,
    pub driver_type: DriverType,
    /// WebSocket URL of the integration driver.
    pub driver_url: String,
    /// Optional driver authentication token.
    ///
    /// Note: the token will not be returned to external clients! See `pwd_protected` if a token is
    /// required.
    pub token: Option<String>,
    /// Authentication method if token is used.
    pub auth_method: Option<WsAuthentication>,
    /// Driver requires a connection password.
    /// This field is usually only set if authentication is required
    pub pwd_protected: Option<bool>,
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
    /// Number of integration instances.
    pub instance_count: Option<u16>,
    /// Driver configuration metadata describing configuration parameters for the web-configurator.
    #[cfg(feature = "sqlx")]
    pub setup_data_schema: Json<Value>,
    #[cfg(not(feature = "sqlx"))]
    pub setup_data_schema: Value,
    /// Release date of the driver.
    pub release_date: Option<NaiveDate>,
    /// Current state. `Idle` if the driver is not in use.
    pub driver_state: Option<DriverState>,
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
    pub pwd_protected: Option<bool>,
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
    /// The driver manifest is only used for registering external drivers. It cannot be updated.
    pub manifest: Option<DriverManifest>,
    pub release_date: Option<NaiveDate>,
}

/// Integration driver type.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum DriverType {
    /// Pre-installed integration in the firmware.
    Local,
    /// External integration on the network.
    External,
    /// Custom installed integration on the remote.
    Custom,
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

/// The driver manifest contains required and optional features for the driver to work.
///
/// Furthermore, it can also contain additional metadata for the Core service,
/// which is required for certain features.
/// For example OAuth2 configuration data like the authorization and token endpoints.
/// This data may only be transmitted to the core, but won't be exposed in the driver
/// management API endpoints.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize, Validate)]
pub struct DriverManifest {
    /// Required features of an integration driver.
    #[validate]
    #[validate(length(min = 1))]
    pub features: Option<Vec<DriverFeature>>,
    pub iot_class: Option<IotClass>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Validate)]
pub struct DriverFeature {
    /// Property specifying a single hardware or software feature used by the driver.
    /// Valid properties are documented in the integration-API.
    #[validate(length(min = 4, max = 50, message = "Invalid length (4..50)"))]
    pub name: String,
    /// Optional or required feature
    /// - `true`: (default) indicates that the driver can't function, or isn't designed to function,
    ///            when the specified feature isn't present on the Remote.
    /// - `false`: indicates that the driver uses the feature if present on the Remote, but that it
    ///            is designed to function without the specified feature if necessary.
    pub required: Option<bool>,
    /// Optional feature specific data
    #[cfg(feature = "sqlx")]
    pub data: Option<Json<Value>>,
    #[cfg(not(feature = "sqlx"))]
    pub data: Option<Value>,
}

/// How the integration connects and communicates with a device or service.
///
/// Inspired by [Home Assistant IoT class](https://www.home-assistant.io/blog/2016/02/12/classifying-the-internet-of-things/#classifiers).
#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "snake_case"))]
pub enum IotClass {
    AssumedState,
    CloudPolling,
    CloudPush,
    LocalPolling,
    LocalPush,
}

impl From<IntegrationDriver> for IntegrationDriverUpdate {
    fn from(drv: IntegrationDriver) -> Self {
        Self {
            driver_id: Some(drv.driver_id),
            name: Some(drv.name),
            driver_url: Some(drv.driver_url),
            token: drv.token,
            auth_method: drv.auth_method,
            pwd_protected: drv.pwd_protected,
            version: Some(drv.version),
            min_core_api: drv.min_core_api,
            icon: drv.icon,
            enabled: Some(drv.enabled),
            description: drv.description,
            developer: drv.developer,
            home_page: drv.home_page,
            device_discovery: Some(drv.device_discovery),
            setup_data_schema: Some(drv.setup_data_schema),
            manifest: None,
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
    /// Integration state.
    pub device_state: Option<DeviceState>,
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
#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum DeviceState {
    Unknown,
    Connecting,
    Connected,
    Disconnected,
    Error,
}

/// Integration driver states.
///
/// The intermediate states `Connected` (but not yet authenticated) and `Disconnecting` are omitted.
/// These states are usually of very short nature and are therefore not reported.
#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum DriverState {
    NotConfigured,
    Idle,
    Connecting,
    Active,
    Reconnecting,
    Error,
}

/// Integration states.
///
/// Variants will be serialized in `SCREAMING_SNAKE_CASE`.
#[derive(Debug, Clone, Copy, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationState {
    NotConfigured,
    Unknown,
    Idle,
    Connecting,
    Connected,
    Disconnected,
    Reconnecting,
    Active,
    Error,
}
