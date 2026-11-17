// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Common entity-related data structures used in the Core & Integration APIs.
//!
//! See [Remote Two/3 API documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc)
//! for more information, especially the [entity documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc/entities).
//!
//! All variants will be serialized in `snake_case`.

use crate::{
    REGEX_HTTP_HEADER_NAME, REGEX_ID_CHARS, REGEX_LANGUAGE_CODE, REGEX_WEBVIEW_BACKGROUND_COLOR,
    REGEX_WEBVIEW_CERTIFICATE_SHA256,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;
use strum_macros::*;
use validator::{Validate, ValidationError};

pub const DEF_VOICE_SAMPLE_RATE: u32 = 16000;

/// Supported entity types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    IrEmitter,
    Select,
    Number,
    VoiceAssistant,
    /// Web view entity. Its commands are executed by the user interface of the Remote.
    Webview,
}

/// Button features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonFeature {
    Press,
}

/// Button entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonCommand {
    Push,
}

/// Button entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ButtonAttribute {
    State,
}

/// Switch features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchFeature {
    OnOff,
    Toggle,
}

/// Switch entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchCommand {
    On,
    Off,
    Toggle,
}

/// Switch entity device classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchDeviceClass {
    /// The switch represents a switchable power outlet.
    Outlet,
    /// Generic switch.
    Switch,
}

/// Switch entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchOptionField {
    Readable,
}

/// Switch entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SwitchAttribute {
    State,
}

/// Climate entity features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Climate entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ClimateOptionField {
    /// The unit of temperature measurement: `CELSIUS`, `FAHRENHEIT`.
    /// If not specified, the remote settings are used.
    TemperatureUnit,
    /// Step value for the UI for setting the target temperature.
    /// Defaults: `CELSIUS` = `0.5`, `FAHRENHEIT` = `1`. Smallest step size: `0.1`
    TargetTemperatureStep,
    /// Maximum temperature to show in the UI for the target temperature range.
    MaxTemperature,
    /// Minimum temperature to show in the UI for the target temperature range.
    MinTemperature,
    //FanModes Not yet implemented
}

/// Climate entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum CoverAttribute {
    State,
    Position,
    TiltPosition,
}

/// Light entity features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightCommand {
    On,
    Off,
    Toggle,
}

/// Light entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum LightOptionField {
    ColorTemperatureSteps,
}

/// Light entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerFeature {
    /// The media player can be switched on and off.
    OnOff,
    /// The media player's power state can be toggled.
    Toggle,
    /// The volume level can be set to a specific level.
    Volume,
    /// The volume can be adjusted up (louder) and down.
    VolumeUpDown,
    /// The mute state can be toggled.
    MuteToggle,
    /// The volume can be muted.
    Mute,
    /// The volume can be un-muted.
    Unmute,
    /// The player supports starting and pausing media playback.
    PlayPause,
    /// The player supports stopping media playback.
    Stop,
    /// The player supports skipping to the next track.
    Next,
    /// The player supports returning to the previous track.
    Previous,
    /// The player supports fast-forwarding the current track.
    FastForward,
    /// The player supports rewinding the current track.
    Rewind,
    /// The current track or playlist can be repeated.
    Repeat,
    /// The player supports random playback / shuffling the current playlist.
    Shuffle,
    /// The player supports seeking the playback position.
    Seek,
    /// The player announces the duration of the current media being played.
    MediaDuration,
    /// The player announces the current position of the media being played.
    MediaPosition,
    /// The player announces the media title.
    MediaTitle,
    /// The player announces the media artist.
    MediaArtist,
    /// The player announces the media album if music is being played.
    MediaAlbum,
    /// The player provides an image url of the media being played.
    MediaImageUrl,
    /// The player announces the content type of media being played.
    MediaType,
    /// Directional pad navigation provides cursor_up, _down, _left, _right, _enter commands.
    #[serde(rename = "dpad")]
    #[strum(serialize = "dpad")]
    DPad,
    /// Number pad, provides digit_0 .. digit_9 commands.
    Numpad,
    /// Home navigation support with home and back commands.
    Home,
    /// Menu navigation support with menu and back commands.
    Menu,
    /// Context menu (for example, right-clicking or long pressing an item).
    ContextMenu,
    /// Program guide support with guide and back commands.
    Guide,
    /// Information popup / menu support with info and back commands.
    Info,
    /// Color button support for function_red, _green, _yellow, _blue commands.
    ColorButtons,
    /// Channel zapping support with channel_up and _down commands.
    ChannelSwitcher,
    /// Media playback sources or inputs can be selected.
    SelectSource,
    /// Sound modes can be selected, e.g., stereo or surround.
    SelectSoundMode,
    /// The media can be ejected, e.g., a slot-in CD or USB stick.
    Eject,
    /// The player supports opening and closing, e.g., a disc tray.
    OpenClose,
    /// The player supports selecting or switching the audio track.
    AudioTrack,
    /// The player supports selecting or switching subtitles.
    Subtitle,
    /// The player has recording capabilities with record, my_recordings, live commands.
    Record,
    /// The player supports a settings menu.
    Settings,
    /// The player supports playing a specific media item.
    PlayMedia,
    /// The player supports the play_media action parameter to either play or enqueue.
    PlayMediaAction,
    /// The player allows clearing the active playlist.
    ClearPlaylist,
    /// The player supports browsing media containers.
    BrowseMedia,
    /// The player supports searching for media items.
    SearchMedia,
    /// The player provides a list of media classes as filter for searches.
    SearchMediaClasses,
}

/// Media player entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum MediaPlayerCommand {
    /// Switch on media player.
    On,
    /// Switch off media player.
    Off,
    /// Toggle the current power state.
    Toggle,
    /// Toggle play / pause.
    PlayPause,
    /// Stop playback.
    Stop,
    /// Go back to previous track.
    Previous,
    /// Skip to next track.
    Next,
    /// Fast forward current track.
    FastForward,
    /// Rewind current track.
    Rewind,
    /// Seek to given position in current track.
    Seek,
    /// Set volume to given level.
    Volume,
    /// Increase volume.
    VolumeUp,
    /// Decrease volume.
    VolumeDown,
    /// Toggle mute state.
    MuteToggle,
    /// Mute volume.
    Mute,
    /// Unmute volume.
    Unmute,
    /// Repeat track or playlist.
    Repeat,
    /// Shuffle playlist or start random playback.
    Shuffle,
    /// Channel up.
    ChannelUp,
    /// Channel down.
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
    /// Number pad digit 0
    Digit_0,
    /// Number pad digit 1
    Digit_1,
    /// Number pad digit 2
    Digit_2,
    /// Number pad digit 3
    Digit_3,
    /// Number pad digit 4
    Digit_4,
    /// Number pad digit 5
    Digit_5,
    /// Number pad digit 6
    Digit_6,
    /// Number pad digit 7
    Digit_7,
    /// Number pad digit 8
    Digit_8,
    /// Number pad digit 9
    Digit_9,
    /// Function red
    FunctionRed,
    /// Function green
    FunctionGreen,
    /// Function yellow
    FunctionYellow,
    /// Function blue
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
    /// Start, stop, or open recording menu (device dependant).
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
    /// Play or enqueue a media item.
    PlayMedia,
    /// Remove all items from the playback queue. Current playback behavior is integration-dependent (keep playing the current item or clearing everything).
    ClearPlaylist,
}

/// Media player entity device classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Media player entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerOptionField {
    /// Additional commands the media-player supports, which are not covered in the feature list.
    SimpleCommands,
    /// Number of available volume steps for the set volume command and UI controls.
    VolumeSteps,
    /// Bit-field indicating if the `browse` and `search` commands support stable media IDs:
    ///
    /// - Bit 0: `browse` always returns stable ids.
    /// - Bit 1: `browse` supports stable ids with the `stable_ids` parameter.
    /// - Bit 2: `search` always returns stable ids.
    /// - Bit 3: `search` returns stable ids with the `stable_ids` parameter.
    ///
    /// Default if not provided: `browse` and `search` always return stable IDs (Bit 0 | Bit 2)
    StableIdSupport,
}

/// Media player media content types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaContentType {
    Album,
    App,
    Apps,
    Artist,
    Channel,
    Channels,
    Composer,
    Episode,
    Game,
    Genre,
    Image,
    Movie,
    Music,
    Playlist,
    Podcast,
    Radio,
    Season,
    Track,
    TvShow,
    Url,
    Video,
    // Custom content type of the integration driver
    Other(String),
}

impl MediaContentType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for MediaContentType {
    fn as_ref(&self) -> &str {
        match self {
            MediaContentType::Album => "album",
            MediaContentType::App => "app",
            MediaContentType::Apps => "apps",
            MediaContentType::Artist => "artist",
            MediaContentType::Channel => "channel",
            MediaContentType::Channels => "channels",
            MediaContentType::Composer => "composer",
            MediaContentType::Episode => "episode",
            MediaContentType::Game => "game",
            MediaContentType::Genre => "genre",
            MediaContentType::Image => "image",
            MediaContentType::Movie => "movie",
            MediaContentType::Music => "music",
            MediaContentType::Playlist => "playlist",
            MediaContentType::Podcast => "podcast",
            MediaContentType::Radio => "radio",
            MediaContentType::Season => "season",
            MediaContentType::Track => "track",
            MediaContentType::TvShow => "tv_show",
            MediaContentType::Url => "url",
            MediaContentType::Video => "video",
            MediaContentType::Other(s) => s.as_str(),
        }
    }
}

impl From<&str> for MediaContentType {
    fn from(s: &str) -> Self {
        match s {
            "album" => MediaContentType::Album,
            "app" => MediaContentType::App,
            "apps" => MediaContentType::Apps,
            "artist" => MediaContentType::Artist,
            "channel" => MediaContentType::Channel,
            "channels" => MediaContentType::Channels,
            "composer" => MediaContentType::Composer,
            "episode" => MediaContentType::Episode,
            "game" => MediaContentType::Game,
            "genre" => MediaContentType::Genre,
            "image" => MediaContentType::Image,
            "movie" => MediaContentType::Movie,
            "music" => MediaContentType::Music,
            "playlist" => MediaContentType::Playlist,
            "podcast" => MediaContentType::Podcast,
            "radio" => MediaContentType::Radio,
            "season" => MediaContentType::Season,
            "track" => MediaContentType::Track,
            "tv_show" => MediaContentType::TvShow,
            "url" => MediaContentType::Url,
            "video" => MediaContentType::Video,
            other => MediaContentType::Other(other.to_owned()),
        }
    }
}

impl Serialize for MediaContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MediaContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(MediaContentType::from(&s[..]))
    }
}

/// Media item classes.
#[derive(Debug, Clone, PartialEq, Eq, Display, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaClass {
    Album,
    App,
    Artist,
    Channel,
    Composer,
    Directory,
    Episode,
    Game,
    Genre,
    Image,
    Movie,
    Music,
    Playlist,
    Podcast,
    Radio,
    Season,
    Track,
    TvShow,
    Url,
    Video,
    // Custom media class of the integration driver
    Other(String),
}

impl MediaClass {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for MediaClass {
    fn as_ref(&self) -> &str {
        match self {
            MediaClass::Album => "album",
            MediaClass::App => "app",
            MediaClass::Artist => "artist",
            MediaClass::Channel => "channel",
            MediaClass::Composer => "composer",
            MediaClass::Directory => "directory",
            MediaClass::Episode => "episode",
            MediaClass::Game => "game",
            MediaClass::Genre => "genre",
            MediaClass::Image => "image",
            MediaClass::Movie => "movie",
            MediaClass::Music => "music",
            MediaClass::Playlist => "playlist",
            MediaClass::Podcast => "podcast",
            MediaClass::Radio => "radio",
            MediaClass::Season => "season",
            MediaClass::Track => "track",
            MediaClass::TvShow => "tv_show",
            MediaClass::Url => "url",
            MediaClass::Video => "video",
            MediaClass::Other(s) => s.as_str(),
        }
    }
}

impl From<&str> for MediaClass {
    fn from(s: &str) -> Self {
        match s {
            "album" => MediaClass::Album,
            "app" => MediaClass::App,
            "artist" => MediaClass::Artist,
            "channel" => MediaClass::Channel,
            "composer" => MediaClass::Composer,
            "directory" => MediaClass::Directory,
            "episode" => MediaClass::Episode,
            "game" => MediaClass::Game,
            "genre" => MediaClass::Genre,
            "image" => MediaClass::Image,
            "movie" => MediaClass::Movie,
            "music" => MediaClass::Music,
            "playlist" => MediaClass::Playlist,
            "podcast" => MediaClass::Podcast,
            "radio" => MediaClass::Radio,
            "season" => MediaClass::Season,
            "track" => MediaClass::Track,
            "tv_show" => MediaClass::TvShow,
            "url" => MediaClass::Url,
            "video" => MediaClass::Video,
            other => MediaClass::Other(other.to_owned()),
        }
    }
}

impl Serialize for MediaClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MediaClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(MediaClass::from(&s[..]))
    }
}

/// A media item which can be browsed or played.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct BrowseMediaItem {
    /// Unique identifier of the item. May only be empty for special items like `root` or non-playable directories.
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub media_id: String,
    /// Display name.
    #[validate(length(min = 1, max = 255, message = "Invalid length (1..255)"))]
    pub title: String,
    /// Optional subtitle
    #[validate(length(min = 1, max = 255, message = "Invalid length (1..255)"))]
    pub subtitle: Option<String>,
    /// Artist name.
    #[validate(length(min = 1, max = 255, message = "Invalid length (1..255)"))]
    pub artist: Option<String>,
    /// Album name.
    #[validate(length(min = 1, max = 255, message = "Invalid length (1..255)"))]
    pub album: Option<String>,
    /// Media class for further browse, search, or playback actions.
    pub media_class: Option<MediaClass>,
    /// Media content type for further browse, search, or playback actions.
    pub media_type: Option<MediaContentType>,
    /// If `true`, the item can be browsed (is a container) by using `media_id` and `media_type`.
    pub can_browse: Option<bool>,
    /// If `true`, the item can be played directly by using `media_id` and `media_type`.
    pub can_play: Option<bool>,
    /// If `true`, a search can be performed on the item by using `media_id` and `media_type`.
    pub can_search: Option<bool>,
    /// URL to download the media artwork, or a base64 encoded PNG or JPG image.
    ///
    /// The preferred size is 480x480 pixels.
    /// Use the following URI prefix to use a provided icon: `icon://uc:`, for example, `icon://uc:music`.
    /// Please use a URL whenever possible. Encoded images should be as small as possible.
    #[validate(length(min = 1, max = 32768, message = "Invalid length (1..32768)"))]
    pub thumbnail: Option<String>,
    /// Duration in seconds.
    pub duration: Option<u32>,
    /// Child items if this item is a container.
    /// Child items may not contain further child items (only one level of nesting is supported).
    /// A new browse request must be sent for deeper levels.
    #[validate(nested)]
    pub items: Option<Vec<BrowseMediaItem>>,
}

/// A media item returned as a search result.
///
/// Currently identical in shape to [BrowseMediaItem].
/// Defined as type to allow search-specific fields (e.g. relevance score) to be added
/// in the future without a major breaking change.
pub type SearchMediaItem = BrowseMediaItem;

/// Media play actions.
#[derive(Debug, Clone, PartialEq, Eq, Display, VariantNames, Default)] // strum_macros
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaPlayAction {
    #[default]
    PlayNow,
    PlayNext,
    AddToQueue,
    Other(String),
}

impl MediaPlayAction {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for MediaPlayAction {
    fn as_ref(&self) -> &str {
        match self {
            MediaPlayAction::PlayNow => "PLAY_NOW",
            MediaPlayAction::PlayNext => "PLAY_NEXT",
            MediaPlayAction::AddToQueue => "ADD_TO_QUEUE",
            MediaPlayAction::Other(s) => s.as_str(),
        }
    }
}

impl From<&str> for MediaPlayAction {
    fn from(s: &str) -> Self {
        match s {
            "PLAY_NOW" => MediaPlayAction::PlayNow,
            "PLAY_NEXT" => MediaPlayAction::PlayNext,
            "ADD_TO_QUEUE" => MediaPlayAction::AddToQueue,
            other => MediaPlayAction::Other(other.to_owned()),
        }
    }
}

impl Serialize for MediaPlayAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MediaPlayAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(MediaPlayAction::from(&s[..]))
    }
}

/// Media player repeat modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaPlayerRepeatMode {
    Off,
    All,
    One,
}

/// Media player entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MediaPlayerAttribute {
    /// State of the media player, influenced by the play and power commands.
    State,
    /// Current volume level.
    Volume,
    /// Flag if the volume is muted.
    Muted,
    /// Current media position in seconds.
    MediaPosition,
    /// Optional timestamp when `media_position` was last updated. Requires integration support.
    MediaPositionUpdatedAt,
    /// Media duration in seconds.
    MediaDuration,
    /// Currently playing media information.
    MediaTitle,
    /// Currently playing media information.
    MediaArtist,
    /// Currently playing media information.
    MediaAlbum,
    /// The content ID of media being played.
    MediaId,
    /// The content type of media being played.
    MediaType,
    /// URL to retrieve the album art or an image representing what's being played.
    MediaImageUrl,
    MediaImageUrlSmall,
    MediaImageUrlMedium,
    MediaImageUrlLarge,
    /// Title of Playlist currently playing.
    MediaPlaylist,
    /// Current repeat mode.
    Repeat,
    /// Shuffle mode on or off.
    Shuffle,
    /// Currently selected media or input source.
    Source,
    /// Available media or input sources.
    SourceList,
    /// Currently selected sound mode.
    SoundMode,
    /// Available sound modes.
    SoundModeList,
    /// List of media classes to use as a filter for `search_media`. Custom classes should be avoided.
    SearchMediaClasses,
    /// List of supported [MediaPlayAction].
    PlayMediaAction,
}

/// Sensor entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SensorOptionField {
    /// Label for a custom sensor if `device_class` is not specified or to override a default unit.
    CustomLabel,
    /// Unit label for a custom sensor if `device_class` is not specified or to override a default
    /// unit.
    CustomUnit,
    /// The sensor's native unit of measurement to perform automatic conversion. Applicable to
    /// device classes: `temperature`.
    NativeUnit,
    /// Number of decimal places to show in the UI if the sensor provides the measurement as a
    /// number. Not applicable to string values and the `timestamp` and `date` device classes.
    /// A `duration` value is rounded to this number of decimal places before it is formatted as
    /// a time span.
    Decimals,
}

/// Sensor entity device classes.
///
/// The `timestamp`, `date` and `duration` device classes are formatted by the UI as a date, time
/// or time span, with the localization settings of the Remote. The value must be sent in the
/// specified machine-readable format and must not be formatted for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Calendar date without a time. Value: text in ISO 8601 format `YYYY-MM-DD`, e.g.
    /// `2026-11-15`.
    Date,
    /// Time span, default unit: s. Supported units: `ms`, `s`, `min`, `h`, `d`. Value: number in
    /// the unit of the sensor.
    ///
    /// A value in `min`, `h` or `d` is shown as a time span in the unit of the sensor and the next
    /// smaller unit, e.g. `1.5` hours as `1h 30m`. A value in `ms` or `s` is shown as a number
    /// with the unit.
    Duration,
    /// Energy in kilowatt-hour
    Energy,
    /// Humidity in %
    Humidity,
    /// Power in watt or kilowatt
    Power,
    /// Temperature with automatic °C, °F conversion, depending on remote settings.
    /// Use native_unit option if the temperature is measured in °F.
    Temperature,
    /// Point in time. Value: text in RFC 3339 format, including the UTC offset, e.g.
    /// `2026-11-15T04:23:31+00:00`.
    Timestamp,
    /// Voltage in volt
    Voltage,
    /// Binary sensor. The specific binary device class is stored in the `unit` attribute.
    Binary,
}

/// Sensor entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SensorAttribute {
    State,
    Value,
    Unit,
}

/// Activity features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityFeature {
    OnOff,
    Start,
    Stop,
}

/// Activity entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityCommand {
    On,
    /// Turn off an activity by running the off-sequence
    Off,
    /// Start an activity by running the on-sequence
    Start,
    /// Abort a running on- or off-sequence
    Stop,
}

/// Macro features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroFeature {
    Run,
    Stop,
}

/// Macro entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroCommand {
    /// Run a macro
    Run,
    /// Abort a running macro
    Stop,
}

/// Remote entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteAttribute {
    State,
}

/// IR-emitter entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IrEmitterAttribute {
    State,
}

/// Voice Assistant features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum VoiceAssistantFeature {
    /// Voice command is transcribed and sent back to the Remote in the AssistantEvent::SttResponse event.
    Transcription,
    /// Textual response about the performed action with the AssistantEvent::TextResponse event.
    ResponseText,
    /// Speech response about the performed action with the AssistantEvent::SpeechResponse event.
    ResponseSpeech,
}

/// Voice Assistant entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum VoiceAssistantOptionField {
    AudioCfg,
    Profiles,
    PreferredProfile,
}

/// Voice Assistant entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum VoiceAssistantAttribute {
    State,
}

/// Voice Assistant entity options.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct VoiceAssistantEntityOptions {
    #[validate(nested)]
    pub audio_cfg: Option<AudioConfiguration>,
    #[validate(nested)]
    pub profiles: Option<Vec<VoiceAssistantProfile>>,
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    pub preferred_profile: Option<String>,
}

/// Voice Assistant profile.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Validate)]
pub struct VoiceAssistantProfile {
    /// Profile identifier.
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    #[validate(regex(path = "*REGEX_ID_CHARS"))]
    pub id: String,
    /// Friendly name to show in UI.
    #[validate(length(min = 1, max = 50, message = "Invalid length (max = 50)"))]
    pub name: String,
    /// Optional language code if the profile represents a specific language for speech recognition.
    #[validate(regex(path = "*REGEX_LANGUAGE_CODE"))]
    pub language: Option<String>,
    /// Supported features of this profile if different from the entity features.
    /// This overwrites the entity features. An empty array means "no features".
    pub features: Option<Vec<VoiceAssistantFeature>>,
}

/// Audio stream specification.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AudioConfiguration {
    /// Number of audio channels. Default: 1
    #[validate(range(min = 1, max = 2))]
    #[serde(default = "default_audio_channels")]
    pub channels: u8,
    /// Audio sample rate in Hz.
    /// This should be one of the commonly used sample rates: 8000, 11025, 16000, 22050, 24000, 44100
    /// Other sample rates might not be supported.
    #[validate(range(min = 8000, max = 48000))]
    #[serde(default = "default_audio_sample_rate")]
    pub sample_rate: u32,
    /// Audio sample format.
    #[serde(default)]
    pub sample_format: SampleFormat,
}

fn default_audio_channels() -> u8 {
    1
}

fn default_audio_sample_rate() -> u32 {
    DEF_VOICE_SAMPLE_RATE
}

impl Default for AudioConfiguration {
    fn default() -> Self {
        Self {
            channels: default_audio_channels(),
            sample_rate: default_audio_sample_rate(),
            sample_format: SampleFormat::default(),
        }
    }
}

/// Audio format specification
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SampleFormat {
    #[default]
    /// Signed 16 bit
    I16,
    /// Signed 32 bit
    I32,
    /// Unsigned 16 bit
    U16,
    /// Unsigned 32 bit
    U32,
    /// Float 32 bit
    F32,
}

/// Select entity features. This entity has no features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SelectFeature {}

/// Select entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SelectCommand {
    SelectOption,
    SelectFirst,
    SelectLast,
    SelectNext,
    SelectPrevious,
}

/// Select entity option fields. This entity has no options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SelectOptionField {}

/// Select entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum SelectAttribute {
    State,
    CurrentOption,
    Options,
}

/// Number entity features. The `value` feature is always present, even if not specified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum NumberFeature {
    Value,
    /// The device supports a transition time to reach a new value: `duration` parameter of the
    /// `set_value` command. Optional, only set if supported by the device.
    Transition,
}

/// Number entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum NumberCommand {
    /// Set the value. Parameters: `value`, optional `duration` in milliseconds, which requires the
    /// `transition` feature.
    SetValue,
    /// Increase the value by `step` (defaults to the `step` option), clamped at `max`.
    Increment,
    /// Decrease the value by `step` (defaults to the `step` option), clamped at `min`.
    Decrement,
}

/// Number entity option fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum NumberOptionField {
    /// Minimum value (inclusive). Default: 0, `null`: no lower limit.
    Min,
    /// Maximum value (inclusive). Default: 100, `null`: no upper limit.
    Max,
    /// Step size between values. Default: 1, smallest step size: 0.000001
    Step,
    /// Number of decimal places to show in the UI: 0..6. If not set, the decimal places are
    /// derived from `step`, `min` and `max`.
    Decimals,
    /// Optional unit label of the value, shown next to the value in the UI.
    /// Default: unit of the device class.
    Unit,
    /// Hint for the preferred default representation in the UI. See [NumberDisplayMode].
    DisplayMode,
    /// If false, the current value cannot be read from the device. Default: true
    Readable,
}

/// Number entity device classes.
///
/// The device classes are kept in sync with the sensor entity device classes, unless a class only
/// makes sense for one of them, like the `date` and `timestamp` sensor device classes. There is no
/// automatic unit conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum NumberDeviceClass {
    /// Audio level in decibel, e.g. the volume, bass or treble of an AV receiver.
    AudioLevel,
    /// Battery charge level in %.
    Battery,
    /// Electrical current in ampere.
    Current,
    /// Time span, default unit: s. Supported units: `ms`, `s`, `min`, `h`, `d`.
    Duration,
    /// Energy in kilowatt-hour.
    Energy,
    /// Relative humidity in %.
    Humidity,
    /// Illuminance in lux.
    Illuminance,
    /// Power in watt or kilowatt.
    Power,
    /// Temperature, default unit: °C.
    Temperature,
    /// Voltage in volt.
    Voltage,
}

/// Number entity display mode: hint for the preferred default representation in the UI.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum NumberDisplayMode {
    /// The UI chooses the representation based on the value range.
    #[default]
    Auto,
    /// Prefer a slider. Only applicable if `min` and `max` are set.
    Slider,
    /// Prefer a value field with increment and decrement buttons.
    Stepper,
}

/// Number entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum NumberAttribute {
    State,
    Value,
}

/// Web view entity features. This entity has no features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum WebviewFeature {}

/// Web view entity commands.
///
/// The commands are executed by the user interface of the Remote and are never sent to an
/// integration driver. In the Core-API, the command identifier has the entity type prefix, e.g.
/// `webview.open`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum WebviewCommand {
    /// Wake the display and show the web view fullscreen with its start page.
    Open,
    /// Reload the shown page, in the fullscreen web view or in the widget. No effect if the web
    /// view isn't shown.
    Reload,
    /// Clear the HTTP cache of all web views and reload the shown page.
    ClearCache,
    /// Clear all website data of all web views and the credentials entered in a login dialog,
    /// then reload the shown page.
    ClearData,
}

/// Web view entity option fields. See [WebviewEntityOptions].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum WebviewOptionField {
    /// Start page of the web view: absolute `http://` or `https://` URL. Required.
    Url,
    /// Credentials sent to the origin of the `url` option. See [WebviewAuth].
    Auth,
    /// Exception for an invalid TLS certificate of the host of the `url` option. See [WebviewTls].
    Tls,
    /// Cookies set before the start page is loaded. See [WebviewCookie].
    Cookies,
    /// User agent of the web view. See [WebviewUserAgent].
    UserAgent,
    /// Page zoom in percent: 25..400. Default: 100. Can be overridden by the user.
    Zoom,
    /// Background color while the page is loading, as `#RRGGBB`. Default: background color of
    /// the UI theme.
    BackgroundColor,
    /// Show back, forward, reload and close controls in the fullscreen web view. Default: false.
    /// Can be overridden by the user.
    ShowNavigation,
    /// Reload the page every n seconds while it is shown: 0 = off, otherwise 10..86400.
    /// Default: 0. Can be overridden by the user.
    ReloadInterval,
}

/// Web view entity attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum WebviewAttribute {
    State,
}

/// Web view entity options.
///
/// The origin (scheme, host and port) of the `url` option is the scope of `auth`, `cookies` and
/// `tls`: they are never applied to another origin.
///
/// Secret options: `auth.password`, `auth.token`, `auth.header_value` and `cookies[].value`.
/// The Remote only hands them to its own user interface: Core-API responses omit the secret values
/// and contain the `has_*` flags instead. The derived `Debug` output contains the secret values.
///
/// See [web view entity documentation](https://github.com/unfoldedcircle/core-api/blob/main/doc/entities/entity_webview.md)
/// for more information.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct WebviewEntityOptions {
    /// Start page of the web view. Absolute `http://` or `https://` URL, without user name and
    /// password. The URL is not a secret: don't put tokens in the query string.
    #[validate(length(min = 8, max = 2048, message = "Invalid length (min = 8, max = 2048)"))]
    #[validate(custom(function = "validate_webview_url"))]
    pub url: String,
    /// Credentials sent to the origin of the `url` option.
    #[validate(nested)]
    pub auth: Option<WebviewAuth>,
    /// Exception for an invalid TLS certificate of the host of the `url` option.
    #[validate(nested)]
    pub tls: Option<WebviewTls>,
    /// Cookies set before the start page is loaded, each time the web view is opened. At most 20.
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    #[validate(nested)]
    pub cookies: Option<Vec<WebviewCookie>>,
    /// User agent of the web view. Default: [WebviewUserAgent::Default].
    #[validate(custom(function = "validate_webview_user_agent"))]
    pub user_agent: Option<WebviewUserAgent>,
    /// Page zoom in percent: 25..400. Default: 100.
    #[validate(range(min = 25, max = 400))]
    pub zoom: Option<u16>,
    /// Background color while the page is loading and behind transparent pages, as `#RRGGBB`.
    /// Empty or not set: background color of the UI theme.
    #[validate(regex(path = "*REGEX_WEBVIEW_BACKGROUND_COLOR"))]
    pub background_color: Option<String>,
    /// Show back, forward, reload and close controls in the fullscreen web view. Default: false.
    pub show_navigation: Option<bool>,
    /// Reload the page every n seconds while it is shown: 0 = off, otherwise 10..86400.
    /// Default: 0.
    #[validate(custom(function = "validate_webview_reload_interval"))]
    pub reload_interval: Option<u32>,
}

impl WebviewEntityOptions {
    /// Create web view options with the given start page and default values for all other
    /// options.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            auth: None,
            tls: None,
            cookies: None,
            user_agent: None,
            zoom: None,
            background_color: None,
            show_navigation: None,
            reload_interval: None,
        }
    }
}

/// Web view authentication method.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum WebviewAuthType {
    /// No credentials.
    #[default]
    None,
    /// `username` and `password`, sent as answer to an HTTP authentication request (status 401)
    /// of the origin. Not sent preemptively.
    Basic,
    /// `token`, sent as `Authorization: Bearer <token>` header with every request to the origin.
    Bearer,
    /// `header_name` and `header_value`, sent as header with every request to the origin, e.g. an
    /// API key.
    Header,
}

/// Web view credentials. They are only sent to the origin of the `url` option.
///
/// `password`, `token` and `header_value` are secrets. Core-API responses omit them and contain
/// the `has_password`, `has_token` and `has_header_value` flags instead. In a Core-API update
/// request, a missing secret keeps the stored value and an empty string removes it.
///
/// The length of the secrets is checked without adding the value to the validation error.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[validate(schema(
    function = "validate_webview_auth_secrets",
    skip_on_field_errors = false
))]
pub struct WebviewAuth {
    /// Authentication method.
    #[serde(rename = "type")]
    pub auth_type: WebviewAuthType,
    /// `basic`: user name, at most 256 characters.
    #[validate(length(max = 256, message = "Invalid length (max = 256)"))]
    pub username: Option<String>,
    /// `basic`: password, at most 256 characters. Secret.
    pub password: Option<String>,
    /// Set by the Remote in Core-API responses: a password is set.
    pub has_password: Option<bool>,
    /// `bearer`: token, at most 4096 characters. Secret.
    pub token: Option<String>,
    /// Set by the Remote in Core-API responses: a token is set.
    pub has_token: Option<bool>,
    /// `header`: name of the HTTP request header, e.g. `X-API-Key`.
    ///
    /// Not allowed: `Host`, `Cookie`, `Content-Length`, `Content-Type`, `Connection`,
    /// `Transfer-Encoding`, `Upgrade`, `Origin`, `Referer`, and names starting with `Proxy-` or
    /// `Sec-`.
    #[validate(length(min = 1, max = 64, message = "Invalid length (min = 1, max = 64)"))]
    #[validate(regex(path = "*REGEX_HTTP_HEADER_NAME"))]
    #[validate(custom(function = "validate_webview_header_name"))]
    pub header_name: Option<String>,
    /// `header`: value of the HTTP request header, at most 4096 characters. Secret.
    pub header_value: Option<String>,
    /// Set by the Remote in Core-API responses: a header value is set.
    pub has_header_value: Option<bool>,
}

/// Exception for an invalid TLS certificate of the host of the `url` option, e.g. a self-signed
/// certificate. The exception only applies to page loads from this host.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct WebviewTls {
    /// SHA-256 fingerprint of the expected DER-encoded server certificate: 64 hexadecimal
    /// digits, optionally separated by `:`, case-insensitive. An invalid certificate is only
    /// accepted if its fingerprint matches.
    #[validate(regex(path = "*REGEX_WEBVIEW_CERTIFICATE_SHA256"))]
    pub certificate_sha256: Option<String>,
    /// Accept any invalid certificate of the host. Only used if `certificate_sha256` is not set.
    /// Default: false.
    pub allow_invalid_certificate: Option<bool>,
}

/// Cookie of a web view, set before the start page is loaded, each time the web view is opened.
///
/// The `value` is a secret. Core-API responses omit it and contain `has_value` instead. In a
/// Core-API update request, a cookie without `value` keeps the stored value of the cookie with
/// the same `name`, `domain` and `path`.
///
/// The length of the value is checked without adding the value to the validation error.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[validate(schema(
    function = "validate_webview_cookie_secrets",
    skip_on_field_errors = false
))]
pub struct WebviewCookie {
    /// Cookie name.
    #[validate(length(min = 1, max = 256, message = "Invalid length (min = 1, max = 256)"))]
    pub name: String,
    /// Cookie value according to RFC 6265, at most 4096 characters. Secret. Required in the
    /// entity definition of an integration driver.
    pub value: Option<String>,
    /// Set by the Remote in Core-API responses: a value is set.
    pub has_value: Option<bool>,
    /// Domain of the cookie: the host of the `url` option or a parent domain of it. If not set,
    /// the cookie is only sent to the host of the `url` option.
    #[validate(length(max = 255, message = "Invalid length (max = 255)"))]
    pub domain: Option<String>,
    /// Path of the cookie. Default: `/`.
    #[validate(length(max = 1024, message = "Invalid length (max = 1024)"))]
    pub path: Option<String>,
    /// Only send the cookie over HTTPS. Default: `true` for an `https://` URL, `false` otherwise.
    pub secure: Option<bool>,
    /// Hide the cookie from scripts of the page. Default: false.
    pub http_only: Option<bool>,
    /// Expiry date. If not set, the cookie is a session cookie.
    pub expires: Option<DateTime<Utc>>,
}

impl WebviewCookie {
    /// Create a session cookie for the host of the `url` option.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            has_value: None,
            domain: None,
            path: None,
            secure: None,
            http_only: None,
            expires: None,
        }
    }
}

/// User agent of a web view, sent in the `User-Agent` header and returned in
/// `navigator.userAgent`.
///
/// Serialized as `default`, `mobile`, `desktop` or the custom user agent string.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WebviewUserAgent {
    /// The user agent of the Remote, which identifies the Remote and its firmware version.
    #[default]
    Default,
    /// The user agent of a common mobile browser, e.g. to get the mobile layout of a page.
    Mobile,
    /// The user agent of a common desktop browser, e.g. to get the desktop layout of a page.
    Desktop,
    /// Custom user agent string, at most 256 characters.
    Other(String),
}

impl WebviewUserAgent {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for WebviewUserAgent {
    fn as_ref(&self) -> &str {
        match self {
            WebviewUserAgent::Default => "default",
            WebviewUserAgent::Mobile => "mobile",
            WebviewUserAgent::Desktop => "desktop",
            WebviewUserAgent::Other(s) => s.as_str(),
        }
    }
}

impl From<&str> for WebviewUserAgent {
    fn from(s: &str) -> Self {
        match s {
            "default" => WebviewUserAgent::Default,
            "mobile" => WebviewUserAgent::Mobile,
            "desktop" => WebviewUserAgent::Desktop,
            other => WebviewUserAgent::Other(other.to_owned()),
        }
    }
}

impl Serialize for WebviewUserAgent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WebviewUserAgent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(WebviewUserAgent::from(&s[..]))
    }
}

/// Request headers which can't be set with the `header` web view authentication, in lower case.
/// Names starting with `proxy-` or `sec-` are not allowed either.
const WEBVIEW_FORBIDDEN_HEADER_NAMES: [&str; 9] = [
    "host",
    "cookie",
    "content-length",
    "content-type",
    "connection",
    "transfer-encoding",
    "upgrade",
    "origin",
    "referer",
];

fn validate_webview_url(url: &str) -> Result<(), ValidationError> {
    let has_http_scheme = url
        .get(..7)
        .is_some_and(|s| s.eq_ignore_ascii_case("http://"))
        || url
            .get(..8)
            .is_some_and(|s| s.eq_ignore_ascii_case("https://"));
    if !has_http_scheme {
        return Err(ValidationError::new("url")
            .with_message("Only absolute http:// and https:// URLs are supported".into()));
    }
    let parsed = url::Url::parse(url)
        .map_err(|_| ValidationError::new("url").with_message("Invalid URL".into()))?;
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(ValidationError::new("url").with_message(
            "User name and password are not allowed in the URL, use the auth option".into(),
        ));
    }
    Ok(())
}

fn validate_webview_user_agent(user_agent: &WebviewUserAgent) -> Result<(), ValidationError> {
    let len = user_agent.as_str().chars().count();
    if !(1..=256).contains(&len) {
        return Err(ValidationError::new("length")
            .with_message("Invalid length (min = 1, max = 256)".into()));
    }
    Ok(())
}

fn validate_webview_reload_interval(interval: u32) -> Result<(), ValidationError> {
    if interval == 0 || (10..=86400).contains(&interval) {
        Ok(())
    } else {
        Err(ValidationError::new("range")
            .with_message("Invalid reload interval: 0 = off, otherwise 10..86400".into()))
    }
}

fn validate_webview_header_name(name: &str) -> Result<(), ValidationError> {
    let name = name.to_ascii_lowercase();
    if WEBVIEW_FORBIDDEN_HEADER_NAMES.contains(&name.as_str())
        || name.starts_with("proxy-")
        || name.starts_with("sec-")
    {
        return Err(ValidationError::new("not_allowed")
            .with_message("This header name can't be set".into()));
    }
    Ok(())
}

/// Length check of a secret value.
///
/// The validator derive macros add the field value to a validation error. Secrets are checked
/// here instead, and the error only contains the field name and the maximum length.
fn validate_secret_length(
    field: &'static str,
    value: Option<&String>,
    max: usize,
) -> Result<(), ValidationError> {
    match value {
        Some(value) if value.chars().count() > max => {
            let mut err = ValidationError::new("length")
                .with_message(format!("Invalid length of {field} (max = {max})").into());
            err.add_param("field".into(), &field);
            err.add_param("max".into(), &max);
            Err(err)
        }
        _ => Ok(()),
    }
}

fn validate_webview_auth_secrets(auth: &WebviewAuth) -> Result<(), ValidationError> {
    validate_secret_length("password", auth.password.as_ref(), 256)?;
    validate_secret_length("token", auth.token.as_ref(), 4096)?;
    validate_secret_length("header_value", auth.header_value.as_ref(), 4096)
}

fn validate_webview_cookie_secrets(cookie: &WebviewCookie) -> Result<(), ValidationError> {
    validate_secret_length("value", cookie.value.as_ref(), 4096)
}

#[cfg(test)]
mod tests {
    use crate::{AudioConfiguration, MediaPlayerCommand, MediaPlayerFeature, SampleFormat};
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use validator::Validate;

    // make sure DPad variant is serialized as `dpad` and not as `d_pad`
    #[test]
    fn deserialize_mediaplayer_feature_with_strum() {
        let feature = MediaPlayerFeature::DPad;
        assert_eq!("dpad", feature.as_ref());
        assert_eq!(feature, MediaPlayerFeature::from_str("dpad").unwrap());
    }

    #[test]
    fn number_device_class_and_display_mode_are_snake_case() {
        use crate::{NumberDeviceClass, NumberDisplayMode, NumberOptionField};

        assert_eq!("audio_level", NumberDeviceClass::AudioLevel.as_ref());
        assert_eq!(
            NumberDeviceClass::AudioLevel,
            NumberDeviceClass::from_str("audio_level").unwrap()
        );
        assert_eq!(
            serde_json::json!("audio_level"),
            serde_json::to_value(NumberDeviceClass::AudioLevel).unwrap()
        );
        assert_eq!("display_mode", NumberOptionField::DisplayMode.as_ref());
        assert_eq!("stepper", NumberDisplayMode::Stepper.as_ref());
        assert_eq!(NumberDisplayMode::Auto, NumberDisplayMode::default());
    }

    #[test]
    fn sensor_date_time_device_classes_are_snake_case() {
        use crate::SensorDeviceClass;

        for (name, class) in [
            ("date", SensorDeviceClass::Date),
            ("duration", SensorDeviceClass::Duration),
            ("timestamp", SensorDeviceClass::Timestamp),
        ] {
            assert_eq!(name, class.as_ref());
            assert_eq!(class, SensorDeviceClass::from_str(name).unwrap());
            assert_eq!(
                serde_json::json!(name),
                serde_json::to_value(class).unwrap()
            );
        }
    }

    #[test]
    fn webview_entity_type_is_webview() {
        use crate::EntityType;
        use strum::VariantNames;

        assert_eq!("webview", EntityType::Webview.as_ref());
        assert_eq!("webview", EntityType::Webview.to_string());
        assert_eq!(
            EntityType::Webview,
            EntityType::from_str("webview").unwrap()
        );
        assert_eq!(
            serde_json::json!("webview"),
            serde_json::to_value(EntityType::Webview).unwrap()
        );
        assert_eq!(
            EntityType::Webview,
            serde_json::from_value::<EntityType>(serde_json::json!("webview")).unwrap()
        );
        assert!(EntityType::VARIANTS.contains(&"webview"));
    }

    #[test]
    fn webview_enums_are_snake_case() {
        use crate::{
            WebviewAttribute, WebviewAuthType, WebviewCommand, WebviewFeature, WebviewOptionField,
        };
        use strum::VariantNames;

        assert!(WebviewFeature::VARIANTS.is_empty());

        for (name, cmd) in [
            ("open", WebviewCommand::Open),
            ("reload", WebviewCommand::Reload),
            ("clear_cache", WebviewCommand::ClearCache),
            ("clear_data", WebviewCommand::ClearData),
        ] {
            assert_eq!(name, cmd.as_ref());
            assert_eq!(cmd, WebviewCommand::from_str(name).unwrap());
            assert_eq!(serde_json::json!(name), serde_json::to_value(cmd).unwrap());
            assert_eq!(
                cmd,
                serde_json::from_value::<WebviewCommand>(serde_json::json!(name)).unwrap()
            );
        }
        assert_eq!(4, WebviewCommand::VARIANTS.len());

        assert_eq!(
            &[
                "url",
                "auth",
                "tls",
                "cookies",
                "user_agent",
                "zoom",
                "background_color",
                "show_navigation",
                "reload_interval"
            ],
            WebviewOptionField::VARIANTS
        );
        assert_eq!(
            serde_json::json!("reload_interval"),
            serde_json::to_value(WebviewOptionField::ReloadInterval).unwrap()
        );

        assert_eq!(&["state"], WebviewAttribute::VARIANTS);

        assert_eq!(
            &["none", "basic", "bearer", "header"],
            WebviewAuthType::VARIANTS
        );
        assert_eq!(WebviewAuthType::None, WebviewAuthType::default());
        assert_eq!(
            WebviewAuthType::Bearer,
            serde_json::from_value::<WebviewAuthType>(serde_json::json!("bearer")).unwrap()
        );
    }

    #[test]
    fn webview_user_agent_round_trip() {
        use crate::WebviewUserAgent;

        for (name, user_agent) in [
            ("default", WebviewUserAgent::Default),
            ("mobile", WebviewUserAgent::Mobile),
            ("desktop", WebviewUserAgent::Desktop),
            (
                "MyDashboard/1.0",
                WebviewUserAgent::Other("MyDashboard/1.0".into()),
            ),
        ] {
            assert_eq!(name, user_agent.as_str());
            assert_eq!(user_agent, WebviewUserAgent::from(name));
            assert_eq!(
                serde_json::json!(name),
                serde_json::to_value(&user_agent).unwrap()
            );
            assert_eq!(
                user_agent,
                serde_json::from_value::<WebviewUserAgent>(serde_json::json!(name)).unwrap()
            );
        }
        assert_eq!(WebviewUserAgent::Default, WebviewUserAgent::default());
    }

    #[test]
    fn webview_options_round_trip() {
        use crate::WebviewEntityOptions;

        let examples = [
            // integration driver: bearer token
            serde_json::json!({
                "url": "https://dashboard.example.com/energy?kiosk=1",
                "auth": {
                    "type": "bearer",
                    "token": "eyJhbGciOiJIUzI1NiJ9.example.token"
                },
                "user_agent": "desktop",
                "zoom": 70
            }),
            // integration driver: API key header and cookie
            serde_json::json!({
                "url": "http://inverter.home.arpa/status",
                "auth": {
                    "type": "header",
                    "header_name": "X-API-Key",
                    "header_value": "3b1f0c9e7a"
                },
                "cookies": [
                    {
                        "name": "layout",
                        "value": "compact"
                    }
                ],
                "reload_interval": 60,
                "show_navigation": true
            }),
            // Core-API response: secret values replaced by the has_* flags
            serde_json::json!({
                "url": "https://nas.home.arpa:5001/status",
                "auth": {
                    "type": "basic",
                    "username": "viewer",
                    "has_password": true
                },
                "tls": {
                    "certificate_sha256": "3F:A9:12:5C:77:0B:E4:21:9D:6A:C8:0F:41:B2:93:EE:58:17:0C:D4:2A:9B:61:F0:7E:3C:88:15:A6:D2:4B:C9"
                },
                "cookies": [
                    {
                        "name": "layout",
                        "has_value": true
                    }
                ],
                "zoom": 80,
                "reload_interval": 300
            }),
            // all options
            serde_json::json!({
                "url": "https://dashboard.example.com/",
                "auth": {
                    "type": "none"
                },
                "tls": {
                    "allow_invalid_certificate": true
                },
                "cookies": [
                    {
                        "name": "session",
                        "value": "abc123",
                        "domain": "example.com",
                        "path": "/app",
                        "secure": true,
                        "http_only": true,
                        "expires": "2027-01-01T00:00:00Z"
                    }
                ],
                "user_agent": "MyDashboard/1.0",
                "zoom": 25,
                "background_color": "#1a2B3c",
                "show_navigation": false,
                "reload_interval": 86400
            }),
        ];

        for json in examples {
            let options: WebviewEntityOptions =
                serde_json::from_value(json.clone()).expect("Invalid web view options");
            assert!(
                options.validate().is_ok(),
                "Options should be valid: {:?}",
                options.validate()
            );
            assert_eq!(json, serde_json::to_value(&options).unwrap());
        }
    }

    #[test]
    fn webview_options_new_and_with_options() {
        use crate::intg::AvailableIntgEntity;
        use crate::{
            EntityType, WebviewAuth, WebviewAuthType, WebviewCookie, WebviewEntityOptions,
        };

        let mut options = WebviewEntityOptions::new("https://dashboard.example.com/energy");
        assert_eq!(
            serde_json::json!({ "url": "https://dashboard.example.com/energy" }),
            serde_json::to_value(&options).unwrap()
        );

        options.auth = Some(WebviewAuth {
            auth_type: WebviewAuthType::Bearer,
            token: Some("secret-token".into()),
            ..Default::default()
        });
        options.cookies = Some(vec![WebviewCookie::new("layout", "compact")]);
        options.zoom = Some(70);
        assert!(options.validate().is_ok());

        let entity: AvailableIntgEntity = serde_json::from_value(serde_json::json!({
            "entity_id": "dashboard-energy",
            "entity_type": "webview",
            "name": { "en": "Energy dashboard" }
        }))
        .unwrap();
        let entity = entity.with_options(&options).unwrap();
        assert_eq!(EntityType::Webview, entity.entity_type);
        assert_eq!(
            serde_json::json!({
                "url": "https://dashboard.example.com/energy",
                "auth": { "type": "bearer", "token": "secret-token" },
                "cookies": [{ "name": "layout", "value": "compact" }],
                "zoom": 70
            }),
            serde_json::Value::Object(entity.options.unwrap())
        );
    }

    #[test]
    fn webview_options_valid_edge_cases() {
        use crate::WebviewEntityOptions;

        for json in [
            serde_json::json!({ "url": "HTTPS://Example.com" }),
            serde_json::json!({ "url": "http://192.168.1.10:8123/lovelace/0" }),
            serde_json::json!({ "url": "https://example.com/", "zoom": 400 }),
            serde_json::json!({ "url": "https://example.com/", "reload_interval": 0 }),
            serde_json::json!({ "url": "https://example.com/", "reload_interval": 10 }),
            serde_json::json!({ "url": "https://example.com/", "background_color": "" }),
            serde_json::json!({ "url": "https://example.com/", "auth": { "type": "header", "header_name": "Authorization", "header_value": "Token abc" } }),
            serde_json::json!({ "url": "https://example.com/", "tls": { "certificate_sha256": "3fa9125c770be4219d6ac80f41b293ee58170cd42a9b61f07e3c8815a6d24bc9" } }),
            serde_json::json!({ "url": "https://example.com/", "cookies": [{ "name": "a", "value": "" }] }),
        ] {
            let options: WebviewEntityOptions =
                serde_json::from_value(json.clone()).expect("Invalid web view options");
            assert!(
                options.validate().is_ok(),
                "{json} should be valid: {:?}",
                options.validate()
            );
        }
    }

    #[test]
    fn webview_options_validation_errors() {
        use crate::WebviewEntityOptions;

        let cookies: Vec<_> = (0..21)
            .map(|i| serde_json::json!({ "name": format!("c{i}"), "value": "v" }))
            .collect();

        for (json, field) in [
            (serde_json::json!({ "url": "file:///etc/passwd" }), "url"),
            (serde_json::json!({ "url": "javascript:alert(1)" }), "url"),
            (serde_json::json!({ "url": "http:/example.com/x" }), "url"),
            (serde_json::json!({ "url": "https://" }), "url"),
            (
                serde_json::json!({ "url": "https://user:secret@example.com/" }),
                "url",
            ),
            (
                serde_json::json!({ "url": format!("https://example.com/{}", "a".repeat(2030)) }),
                "url",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "zoom": 24 }),
                "zoom",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "zoom": 401 }),
                "zoom",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "reload_interval": 9 }),
                "reload_interval",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "reload_interval": 86401 }),
                "reload_interval",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "background_color": "red" }),
                "background_color",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "background_color": "#12345" }),
                "background_color",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "user_agent": "" }),
                "user_agent",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "user_agent": "a".repeat(257) }),
                "user_agent",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "tls": { "certificate_sha256": "3F:A9" } }),
                "tls",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "header", "header_name": "Cookie" } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "header", "header_name": "proxy-authorization" } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "header", "header_name": "Sec-Fetch-Mode" } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "header", "header_name": "X API Key" } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "basic", "username": "a".repeat(257) } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "basic", "password": "a".repeat(257) } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "bearer", "token": "a".repeat(4097) } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "auth": { "type": "header", "header_name": "X-Key", "header_value": "a".repeat(4097) } }),
                "auth",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "cookies": cookies }),
                "cookies",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "cookies": [{ "name": "", "value": "v" }] }),
                "cookies",
            ),
            (
                serde_json::json!({ "url": "https://example.com/", "cookies": [{ "name": "a", "value": "v".repeat(4097) }] }),
                "cookies",
            ),
        ] {
            let options: WebviewEntityOptions =
                serde_json::from_value(json.clone()).expect("Invalid web view options");
            let errors = options
                .validate()
                .expect_err(&format!("{json} should be invalid"));
            assert!(
                errors.errors().contains_key(field),
                "{json}: expected error for {field}, got: {errors:?}"
            );
        }
    }

    #[test]
    fn webview_auth_requires_type() {
        use crate::WebviewAuth;

        let res = serde_json::from_value::<WebviewAuth>(serde_json::json!({ "token": "abc" }));
        assert!(res.is_err(), "Missing auth type should fail: {res:?}");
    }

    #[test]
    fn webview_secret_validation_error_omits_the_secret() {
        use crate::{WebviewAuth, WebviewAuthType, WebviewCookie};

        let secret = "s".repeat(4097);
        let auth = WebviewAuth {
            auth_type: WebviewAuthType::Bearer,
            token: Some(secret.clone()),
            ..Default::default()
        };
        let errors = auth.validate().expect_err("Token should be too long");
        assert!(!format!("{errors:?}").contains(&secret));
        assert!(!errors.to_string().contains(&secret));
        assert!(errors.to_string().contains("token"), "{errors}");

        let cookie = WebviewCookie::new("session", secret.clone());
        let errors = cookie.validate().expect_err("Value should be too long");
        assert!(!format!("{errors:?}").contains(&secret));
        assert!(!errors.to_string().contains(&secret));
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

    #[test]
    fn deserialize_valid_audio_cfg() {
        let json =
            serde_json::json!({ "channels": 2, "sample_rate": 24000, "sample_format": "U16" });
        let cfg: AudioConfiguration = serde_json::from_value(json).expect("Invalid json message");

        assert_eq!(2, cfg.channels);
        assert_eq!(24000, cfg.sample_rate);
        assert_eq!(SampleFormat::U16, cfg.sample_format);

        assert!(
            cfg.validate().is_ok(),
            "Audio configuration should be valid"
        )
    }

    #[test]
    fn deserialize_audio_cfg_with_invalid_sample_format() {
        let json = serde_json::json!({ "sample_format": "I8" });
        let res = serde_json::from_value::<AudioConfiguration>(json);

        assert!(
            res.is_err(),
            "Invalid sample format should result in an error, but got: {res:?}"
        );
    }

    #[test]
    fn test_media_play_action() {
        use crate::MediaPlayAction;

        // Standard variants
        let action = MediaPlayAction::PlayNow;
        assert_eq!(action.as_str(), "PLAY_NOW");
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(serialized, "\"PLAY_NOW\"");
        let deserialized: MediaPlayAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);

        // Other variant
        let action = MediaPlayAction::Other("CUSTOM_ACTION".to_owned());
        assert_eq!(action.as_str(), "CUSTOM_ACTION");
        let serialized = serde_json::to_string(&action).unwrap();
        assert_eq!(serialized, "\"CUSTOM_ACTION\"");
        let deserialized: MediaPlayAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, action);

        // From string
        assert_eq!(
            MediaPlayAction::from("PLAY_NEXT"),
            MediaPlayAction::PlayNext
        );
        assert_eq!(
            MediaPlayAction::from("SOME_UNKNOWN"),
            MediaPlayAction::Other("SOME_UNKNOWN".to_owned())
        );
    }
}
