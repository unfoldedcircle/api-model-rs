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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
    Remote,
}

/// Button features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonFeature {
    Press,
}

/// Button entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonCommand {
    Push,
}

/// Button entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonAttribute {
    State,
}

/// Switch features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchFeature {
    OnOff,
    Toggle,
}

/// Switch entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchCommand {
    On,
    Off,
    Toggle,
}

/// Switch entity device classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchOption {
    Readable,
}

/// Switch entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchAttribute {
    State,
}

/// Climate entity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum CoverAttribute {
    State,
    Position,
    TiltPosition,
}

/// Light entity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightCommand {
    On,
    Off,
    Toggle,
}

/// Light entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightOption {
    ColorTemperatureSteps,
}

/// Light entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
    /// Directional pad navigation, provides cursor_up, _down, _left, _right, _enter commands.
    #[serde(rename = "dpad")]
    #[strum(serialize = "dpad")]
    DPad,
    /// Number pad, provides digit_0 .. digit_9 commands.
    Numpad,
    /// Home navigation support with home & back commands.
    Home,
    /// Menu navigation support with menu & back commands.
    Menu,
    /// Context menu (for example right-clicking or long pressing an item).
    ContextMenu,
    /// Program guide support with guide & back commands.
    Guide,
    /// Information popup / menu support with info & back commands.
    Info,
    /// Color button support for function_red, _green, _yellow, _blue commands.
    ColorButtons,
    /// Channel zapping support with channel_up and _down commands.
    ChannelSwitcher,
    /// Media playback sources or inputs can be selected.
    SelectSource,
    /// Sound modes can be selected, e.g. stereo or surround.
    SelectSoundMode,
    /// The media can be ejected, e.g. a slot-in CD or USB stick.
    Eject,
    /// The player supports opening and closing, e.g. a disc tray.
    OpenClose,
    /// The player supports selecting or switching the audio track.
    AudioTrack,
    /// The player supports selecting or switching subtitles.
    Subtitle,
    /// The player has recording capabilities with record, my_recordings, live commands.
    Record,
    /// The player supports a settings menu.
    Settings,
}

/// Media player entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
#[allow(non_camel_case_types)]
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
    ChannelUp,
    ChannelDown,
    /// Directional pad up
    CursorUp,
    /// Directional pad down
    CursorDown,
    /// Directional pad left
    CursorLeft,
    /// Directional pad right
    CursorRight,
    /// Directional pad enter
    CursorEnter,
    Digit_0,
    Digit_1,
    Digit_2,
    Digit_3,
    Digit_4,
    Digit_5,
    Digit_6,
    Digit_7,
    Digit_8,
    Digit_9,
    FunctionRed,
    FunctionGreen,
    FunctionYellow,
    FunctionBlue,
    /// Home menu
    Home,
    /// General menu
    Menu,
    /// Context menu
    ContextMenu,
    /// Program guide menu.
    Guide,
    /// Information menu / what's playing.
    Info,
    /// Back / exit function for menu navigation.
    Back,
    /// Select media playback source or input from the available sources.
    SelectSource,
    /// Select a sound mode from the available modes.
    SelectSoundMode,
    /// Start, stop or open recording menu (device dependant).
    Record,
    /// Open recordings.
    MyRecordings,
    /// Switch to live view.
    Live,
    /// Eject media.
    Eject,
    /// Open or close.
    OpenClose,
    /// Switch or select audio track.
    AudioTrack,
    /// Switch or select subtitle.
    Subtitle,
    /// Settings menu
    Settings,
}

/// Media player entity device classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerDeviceClass {
    /// Audio-video receiver.
    Receiver,
    /// Set-top box for multichannel video and media playback.
    SetTopBox,
    /// Smart speakers or stereo device.
    Speaker,
    /// Device for media streaming services.
    StreamingBox,
    /// Television device.
    TV,
}

/// Media player entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerOption {
    /// Additional commands the media-player supports, which are not covered in the feature list.
    SimpleCommands,
    /// Number of available volume steps for the set volume command and UI controls.
    VolumeSteps,
}

/// Media player media types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Music,
    Radio,
    Tvshow,
    Movie,
    Video,
}

/// Media player repeat modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerRepeatMode {
    Off,
    All,
    One,
}

/// Media player entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
    SourceList,
    SoundMode,
    SoundModeList,
}

/// Sensor entity options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
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
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SensorAttribute {
    State,
    Value,
    Unit,
}

/// Activity features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityFeature {
    OnOff,
    Start,
}

/// Activity entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityCommand {
    On,
    Off,
    Start,
}

/// Macro features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroFeature {
    Run,
}

/// Macro entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroCommand {
    Run,
}

/// Remote features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteFeature {
    OnOff,
    Toggle,
    Send,
    StopSend,
}

/// Remote entity commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteCommand {
    On,
    Off,
    Toggle,
    Send,
    StopSend,
    SendSequence,
}

/// Remote entity attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteAttribute {
    State,
}

#[cfg(test)]
mod tests {
    use crate::{MediaPlayerCommand, MediaPlayerFeature};
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    // make sure DPad variant is serialized as `dpad` and not as `d_pad`
    #[test]
    fn deserialize_mediaplayer_feature_with_strum() {
        let feature = MediaPlayerFeature::DPad;
        assert_eq!("dpad", feature.as_ref());
        assert_eq!(feature, MediaPlayerFeature::from_str("dpad").unwrap());
    }

    #[test]
    fn deserialize_mediaplayer_command_with_strum() {
        let cmd = MediaPlayerCommand::Digit_1;
        assert_eq!("digit_1", cmd.as_ref());
        assert_eq!(cmd, MediaPlayerCommand::from_str("digit_1").unwrap());
    }

    #[derive(Serialize, Deserialize)]
    struct FeatureTest {
        pub feature: MediaPlayerFeature,
    }

    #[test]
    fn deserialize_mediaplayer_feature() {
        let json = serde_json::json!({ "feature": "dpad" });
        let test: FeatureTest = serde_json::from_value(json).expect("Invalid json message");

        assert_eq!(MediaPlayerFeature::DPad, test.feature);
    }

    #[derive(Serialize, Deserialize)]
    struct CommandTest {
        pub cmd: MediaPlayerCommand,
    }

    #[test]
    fn deserialize_mediaplayer_command() {
        let json = serde_json::json!({ "cmd": "digit_0" });
        let test: CommandTest = serde_json::from_value(json).expect("Invalid json message");

        assert_eq!(MediaPlayerCommand::Digit_0, test.cmd);
    }
}
