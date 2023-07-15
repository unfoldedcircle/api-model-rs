// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Common entity related data structures used in the Core & Integration APIs.
//!
//! See [Remote Two API documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc)
//! for more information, especially the [entity documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc/entities).
//!
//! All variants will be serialized in `snake_case`.

use serde::{Deserialize, Serialize};
use strum_macros::*;

/// Supported entity types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
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
    /// Internal entity only at the moment
    Activity,
    /// Internal entity only at the moment
    Macro,
    /// Internal entity only at the moment
    Remote,
}

/// Button features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonFeature {
    Press,
}

/// Button entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonCommand {
    Push,
}

/// Button entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonAttribute {
    State,
}

/// Switch features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchFeature {
    OnOff,
    Toggle,
}

/// Switch entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchCommand {
    On,
    Off,
    Toggle,
}

/// Switch entity device classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchDeviceClass {
    /// The switch represents a switchable power outlet.
    Outlet,
    /// Generic switch.
    Switch,
}

/// Switch entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchOption {
    Readable,
}

/// Switch entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchAttribute {
    State,
}

/// Climate entity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ClimateOption {
    /// The unit of temperature measurement: `CELSIUS`, `FAHRENHEIT`.
    /// If not specified, the remote settings are used.
    TemperatureUnit,
    /// Step value for the UI for setting the target temperature.
    /// Defaults: `CELSIUS` = `0.5`, 'FAHRENHEIT` = `1`. Smallest step size: `0.1`
    TargetTemperatureStep,
    /// Maximum temperature to show in the UI for the target temperature range.
    MaxTemperature,
    /// Minimum temperature to show in the UI for the target temperature range.
    MinTemperature,
    //FanModes Not yet implemented
}

/// Climate entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ClimateCommand {
    On,
    Off,
    HvacMode,
    TargetTemperature,
    // TargetTemperatureRange,
    // FanMode,
}

/// Climate entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ClimateAttribute {
    State,
    CurrentTemperature,
    TargetTemperature,
    TargetTemperatureHigh,
    TargetTemperatureLow,
    FanMode,
}

/// Cover entity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum CoverFeature {
    Open,
    Close,
    Stop,
    Position,
    // Tilt,
    // TiltStop,
    // TiltPosition,
}

/// Cover entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum CoverCommand {
    Open,
    Close,
    Stop,
    Position,
}

/// Cover entity device classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum CoverDeviceClass {
    /// Window blinds or shutters which can be opened, closed or tilted.
    Blind,
    /// Window curtain or drapes which can be opened or closed.
    Curtain,
    /// Controllable garage door.
    Garage,
    /// Sun shades which can be opened to protect an area from the sun.
    Shade,
}

/// Cover entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum CoverAttribute {
    State,
    Position,
    TiltPosition,
}

/// Light entity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightFeature {
    OnOff,
    Toggle,
    Dim,
    Color,
    ColorTemperature,
}

/// Light entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightCommand {
    On,
    Off,
    Toggle,
}

/// Light entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightOption {
    ColorTemperatureSteps,
}

/// Light entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightAttribute {
    State,
    Hue,
    Saturation,
    Brightness,
    ColorTemperature,
}

/// Media player entity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
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

/// Media player entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
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

/// Media player entity device classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerDeviceClass {
    /// Audio-video receiver.
    Receiver,
    /// Smart speakers or stereo device.
    Speaker,
}

/// Media player entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerOption {
    VolumeSteps,
}

/// Media player entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerAttribute {
    State,
    Volume,
    Muted,
    MediaPosition,
    MediaDuration,
    MediaTitle,
    MediaArtist,
    MediaAlbum,
    MediaImageUrl,
    MediaImageUrlSmall,
    MediaImageUrlMedium,
    MediaImageUrlLarge,
    MediaType,
    Repeat,
    Shuffle,
    Source,
    SourceMode,
}

/// Sensor entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
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

/// Sensor entity device classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SensorDeviceClass {
    /// Generic sensor with custom label and unit
    Custom,
    /// Battery charge in %
    Battery,
    /// Electrical current in ampere
    Current,
    /// Energy in kilowatt-hour
    Energy,
    /// Humidity in %
    Humidity,
    /// Power in watt or kilowatt
    Power,
    /// Temperature with automatic °C, °F conversion, depending on remote settings.
    /// Use native_unit option if the temperature is measured in °F.
    Temperature,
    /// Voltage in volt
    Voltage,
}

/// Sensor entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SensorAttribute {
    State,
    Value,
    Unit,
}

/// Activity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityFeature {
    OnOff,
    Start,
}

/// Activity entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityCommand {
    On,
    Off,
    Start,
}

/// Macro features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroFeature {
    Run,
}

/// Macro entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroCommand {
    Run,
}

/// Remote features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteFeature {
    OnOff,
    Send,
    StopSend,
}

/// Remote entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, EnumVariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteCommand {
    On,
    Off,
    Send,
    StopSend,
}
