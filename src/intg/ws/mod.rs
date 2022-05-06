// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration API specific WebSocket messages.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use validator::Validate;

use crate::intg::{AvailableIntgEntity, DeviceState};
use crate::EntityType;

/// Remote Two initiated request messages for the integration driver.
///
/// The corresponding response message name is set with the strum message macro
///
/// # Examples
///
/// Deserialize from string:
/// ```
/// use std::str::FromStr;
/// use uc_api::intg::ws::R2Request;
/// assert_eq!(Ok(R2Request::SubscribeEvents), R2Request::from_str("subscribe_events"));
/// ```
// TODO refactor response message relationship? Use a typed DriverResponse variant as enum data.
//      --> verify Serde if data can be ignored while (de)serializing. This was the reason for using
//          the simple strum message approach...
#[derive(
    Debug,
    Clone,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumMessage,
    strum_macros::EnumString,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum R2Request {
    #[strum(message = "driver_version")]
    GetDriverVersion,
    // returns an event instead of a response message
    GetDeviceState,
    #[strum(message = "device_setup_complete")]
    SetupDevice,
    #[strum(message = "available_entities")]
    GetAvailableEntities,
    #[strum(message = "result")]
    SubscribeEvents,
    #[strum(message = "result")]
    UnsubscribeEvents,
    #[strum(message = "entity_states")]
    GetEntityStates,
    #[strum(message = "result")]
    EntityCommand,
}

/// Remote Two response messages for the integration driver.
#[derive(
    Debug,
    Clone,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum R2Response {
    Version,
    SupportedEntityTypes,
    ConfiguredEntities,
    LocalizationCfg,
    SetupUserAction,
}

/// Integration specific events emitted from Remote Two
#[derive(
    Debug,
    Clone,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum R2Event {
    Connect,
    Disconnect,
    EnterStandby,
    ExitStandby,
    StartDiscovery,
    StopDiscovery,
    AbortDeviceSetup,
}

/// Integration driver response messages.
#[derive(
    Debug,
    Clone,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DriverResponse {
    Result,
    DriverVersion,
    DeviceSetupComplete,
    AvailableEntities,
    EntityStates,
}

/// Events emitted from the integration driver
#[derive(
    Debug,
    Clone,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DriverEvent {
    AuthRequired,
    DeviceState,
    DeviceSetupProgress,
    AbortDeviceSetup,
    EntityChange,
    EntityAvailable,
    EntityRemoved,
    DiscoveredDevice,
    DiscoveryFinished,
}

/// Payload data of a `device_state` event message in `msg_data` property.  
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceStateMsgData {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub state: DeviceState,
}

/// Payload data of `entity_available` event message in `msg_data` property.
///
/// This is an optional event and not yet implemented in the core.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityAvailableMsgData {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub features: Option<Vec<String>>,
    pub name: HashMap<String, String>,
    pub area: Option<String>,
}

/// Payload data of `entity_removed` event message in `msg_data` property.
///  
/// This is an optional event and not yet implemented in the core.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityRemovedMsgData {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableEntitiesFilter {
    pub device_id: Option<String>,
    pub entity_type: Option<EntityType>,
}

/// Payload data of `available_entities` response message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AvailableEntitiesMsgData {
    pub filter: Option<AvailableEntitiesFilter>,
    #[validate]
    pub available_entities: Vec<AvailableIntgEntity>,
}
