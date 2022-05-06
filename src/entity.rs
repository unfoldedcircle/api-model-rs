// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Common entity related data structures used in the Core & Integration APIs.
//!
//! See [Remote Two API documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc)
//! for more information, especially the [entity documentation](ttps://github.com/unfoldedcircle/core-api/tree/main/doc/entities).

use serde::{Deserialize, Serialize};

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
    Serialize,
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
    Serialize,
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
    Serialize,
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
    Serialize,
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
    Serialize,
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
    Deserialize,
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
    Deserialize,
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
    Deserialize,
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
    Deserialize,
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
    Deserialize,
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
    Deserialize,
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
    Deserialize,
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
