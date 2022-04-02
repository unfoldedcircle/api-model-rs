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
#[skip_serializing_none]
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
    Debug,
    Clone,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "snake_case"))]
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
#[derive(
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::EnumVariantNames,
    PartialEq,
    Deserialize,
)]
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
#[derive(
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::EnumVariantNames,
    PartialEq,
    Deserialize,
)]
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
#[derive(
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::EnumVariantNames,
    PartialEq,
    Deserialize,
)]
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
#[derive(
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::EnumVariantNames,
    PartialEq,
    Deserialize,
)]
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
#[derive(
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::EnumVariantNames,
    PartialEq,
    Deserialize,
)]
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
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct EntityChange {
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
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AvailableIntgEntity {
    /// Optional associated device, if the integration driver supports multiple devices.
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    pub device_id: Option<String>,
    /// Discriminator value for the concrete entity device type.
    pub entity_type: EntityType,
    /// Unique entity identifier within the integration device.
    #[validate(length(
        min = 1,
        max = 36,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 50)"
    ))]
    pub entity_id: String,
    /// Optional device type. This can be used by the UI to represent the entity with a different
    /// icon, behaviour etc. See entity documentation for available device classes.
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub device_class: Option<String>,
    /// Display name of the entity in the UI.
    /// An english text with key `en` should always be provided as fallback option. Otherwise it's
    /// not guaranteed which text will be displayed if the user selected language is not provided.
    pub name: HashMap<String, String>,
    /// Supported features of the entity.
    /// See entity specific feature enums and the entity documentation for available features.
    pub features: Option<Vec<String>>,
    /// Optional area if supported by the integration. E.g. `Living room`.
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub area: Option<String>,
    /// Optional entity options. See entity documentation for available options.
    pub options: Option<serde_json::Map<String, Value>>,
}

/// Switch entity device classes.
///
/// Variants will be serialized in `snake_case`.
/// See API documentation for more information.
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
#[derive(
    Debug,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumVariantNames,
    PartialEq,
    Serialize,
)]
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
