// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

//! Entity related data structures.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

/// Execute an entity command.
///
/// Instruct the integration driver to execute a command like "turn on" or "change temperature".
/// Optional command data like temperature value or channel number can be provided in the `params` array.
/// The parameter objects are described in the entity feature definitions.
///
/// After successfully executing a command, the remote expects an `entity_change` event with the updated feature
/// value(s). The immediate `result` response is to acknowledge the command or to return any immediate failures in
/// case the driver already knows it's unable to perform the command due to device communication issues etc.
#[derive(Debug, Deserialize)]
pub struct EntityCommand {
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub cmd_id: String,
    pub params: Option<Value>,
}

/// Supported entity types.
///
/// Variants will be serialized in `snake_case`.
#[derive(
    Debug, strum_macros::Display, strum_macros::EnumString, PartialEq, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Button,
    Switch,
    Climate,
    Cover,
    Light,
    MediaPlayer,
    Sensor,
}

/// Switch entity commands.
///
/// Variants will be serialized in `snake_case`.
#[derive(strum_macros::EnumString, PartialEq, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SwitchCommand {
    On,
    Off,
    Toggle,
}

/// Climate entity commands.
///
/// Variants will be serialized in `snake_case`.
#[derive(strum_macros::EnumString, PartialEq, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ClimateCommand {
    On,
    Off,
    HvacMode,
    TargetTemperature,
    // TargetTemperatureRange,
    // FanMode,
}

/// Cover entity commands.
///
/// Variants will be serialized in `snake_case`.
#[derive(strum_macros::EnumString, PartialEq, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CoverCommand {
    Open,
    Close,
    Stop,
    Position,
}

/// Light entity commands.
///
/// Variants will be serialized in `snake_case`.
#[derive(strum_macros::EnumString, PartialEq, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LightCommand {
    On,
    Off,
    Toggle,
}

/// Media player entity commands.
///
/// Variants will be serialized in `snake_case`.
#[derive(strum_macros::EnumString, PartialEq, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MediaPlayerCommand {
    On,
    Off,
    Toggle,
    PlayPause,
    Stop,
    Previous,
    Next,
    FastForward,
    Rewind,
    Seek,
    Volume,
    VolumeUp,
    VolumeDown,
    MuteToggle,
    Mute,
    Unmute,
    Repeat,
    Shuffle,
}

/// Entity state change event.
///
/// Emitted when an attribute of an entity changes, e.g. is switched off. Either after an `entity_command` or if the
/// entity is updated manually through a user or an external system. This keeps the remote in sync with the real
/// state of the entity without the need of constant polling.
#[derive(Debug, Serialize)]
pub struct EntityChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub attributes: serde_json::Map<String, Value>,
}

/// Available entity definition provided by an integration.
///
/// The `entity_type` value acts as discriminator for the entity type, which defines the supported
/// features and options of an entity.
/// E.g. a simple `button` entity supports on / off, whereas a `cover` entity can be a simple window
/// cover with only the ability to open / close, or a venetian blind with position and tilting
/// features.
///
/// See entity documentation for more information about the individual entity features and options.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct AvailableEntity {
    /// Optional associated device, if the integration driver supports multiple devices.
    pub device_id: Option<String>,
    /// Discriminator value for the concrete entity device type.
    pub entity_type: EntityType,
    /// Unique entity identifier.
    pub entity_id: String,
    /// Optional device type. This can be used by the UI to represent the entity with a different
    /// icon, behaviour etc. See entity documentation for available device classes.
    pub device_class: Option<String>,
    /// Display name of the entity in the UI.
    /// An english text should always be provided as fallback option.
    pub friendly_name: HashMap<String, String>,
    /// Supported features of the entity. See entity documentation for available features.
    pub features: Option<Vec<String>>,
    pub area: Option<String>,
    /// Optional entity options. See entity documentation for available options.
    pub options: Option<serde_json::Map<String, Value>>,
}

/// Switch entity device classes.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SwitchDeviceClass {
    Outlet,
    Switch,
}

/// Climate entity features.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ClimateFeature {
    OnOff,
    Heat,
    Cool,
    CurrentTemperature,
    TargetTemperature,
    //TargetTemperatureRange Not yet implemented
    //Fan Not yet implemented
}

/// Climate entity options.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ClimateOption {
    TemperatureUnit,
    TargetTemperatureStep,
    MaxTemperature,
    MinTemperature,
    //FanModes Not yet implemented
}

/// Cover entity features.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CoverFeature {
    Open,
    Close,
    Stop,
    Position,
    // Tilt,
    // TiltStop,
    // TiltPosition,
}

/// Light entity features.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LightFeature {
    OnOff,
    Toggle,
    Dim,
    Color,
    ColorTemperature,
}

/// Media player entity features.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MediaPlayerFeature {
    OnOff,
    Toggle,
    Volume,
    VolumeUpDown,
    MuteToggle,
    Mute,
    Unmute,
    PlayPause,
    Stop,
    Next,
    Previous,
    FastForward,
    Rewind,
    Repeat,
    Shuffle,
    Seek,
    MediaDuration,
    MediaPosition,
    MediaTitle,
    MediaArtist,
    MediaAlbum,
    MediaImageUrl,
    MediaType,
}

/// Sensor entity options.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(Debug, strum_macros::Display, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SensorOption {
    /// Label for a custom sensor if `device_class` is not specified or to override a default unit.
    CustomLabel,
    /// Unit label for a custom sensor if `device_class` is not specified or to override a default
    /// unit.
    CustomUnit,
    /// The sensor's native unit of measurement to perform automatic conversion. Applicable to
    /// device classes: `temperature`.
    NativeUnit,
    /// Number of decimal places to show in the UI if the sensor provides the measurement as a
    /// number. Not applicable to string values.
    Decimals,
}
