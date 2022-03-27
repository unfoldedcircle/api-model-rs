// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! Integration driver specific WebSocket messages.

use serde::{Deserialize, Serialize};

/// Remote Two initiated request messages for the integration driver.
///
/// The corresponding response message name is set with the strum message macro
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
