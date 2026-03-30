// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Common entity-related data structures used in the Core & Integration APIs.
//!
//! See [Remote Two/3 API documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc)
//! for more information, especially the [entity documentation](https://github.com/unfoldedcircle/core-api/tree/main/doc/entities).
//!
//! All variants will be serialized in `snake_case`.

use crate::{REGEX_ID_CHARS, REGEX_LANGUAGE_CODE};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;
use strum_macros::*;
use validator::Validate;

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
    VoiceAssistant,
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
    /// number. Not applicable to string values.
    Decimals,
}

/// Sensor entity device classes.
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
}

/// Activity entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum ActivityCommand {
    On,
    Off,
    Start,
}

/// Macro features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroFeature {
    Run,
}

/// Macro entity commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum MacroCommand {
    Run,
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
