// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration API related entity data structures.

use std::collections::HashMap;

use serde::ser::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumString, VariantNames};

use crate::{EntityType, REGEX_ICON_ID, REGEX_ID_CHARS};

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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityCommand {
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub cmd_id: String,
    pub params: Option<serde_json::Map<String, Value>>,
}

/// Entity state change event.
///
/// Emitted when an attribute of an entity changes, e.g., is switched off. Either after an `entity_command` or if the
/// entity is updated manually through a user or an external system. This keeps the remote in sync with the real
/// state of the entity without the need of constant polling.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityChange {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    /// Integration specific entity identifier.
    pub entity_id: String,
    pub attributes: serde_json::Map<String, Value>,
}

/// Voice assistant event.
///
/// The `assistant_event` message must be emitted by the integration driver to start the audio
/// stream and for providing optional feedback about the voice command processing and outcome.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum AssistantEvent {
    /// Integration is ready to receive voice audio stream.
    Ready { entity_id: String, session_id: u32 },
    /// Transcribed text from voice audio stream.
    SttResponse {
        entity_id: String,
        session_id: u32,
        data: AssistantSttResponse,
    },
    /// Textual response about the performed action.
    TextResponse {
        entity_id: String,
        session_id: u32,
        data: AssistantTextResponse,
    },
    /// Speech response about the performed action.
    SpeechResponse {
        entity_id: String,
        session_id: u32,
        data: AssistantSpeechResponse,
    },
    /// Voice processing finished.
    Finished { entity_id: String, session_id: u32 },
    /// Error while sending or processing the audio stream.
    Error {
        entity_id: String,
        session_id: u32,
        data: AssistantError,
    },
}

/// Speech-to-text response event data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantSttResponse {
    pub text: String,
}

impl AssistantSttResponse {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// Textual response event data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantTextResponse {
    pub success: bool,
    pub text: String,
}

impl AssistantTextResponse {
    pub fn new(success: bool, text: impl Into<String>) -> Self {
        Self {
            success,
            text: text.into(),
        }
    }
}

/// Speech response event data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantSpeechResponse {
    pub url: String,
    pub mime_type: String,
}

impl AssistantSpeechResponse {
    pub fn new(url: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            mime_type: mime_type.into(),
        }
    }
}

/// Error event data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantError {
    pub code: AssistantErrorCode,
    /// Optional error message returned from the assistant.
    pub message: String,
}

impl AssistantError {
    pub fn new(code: AssistantErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssistantErrorCode {
    /// Voice assistant service is not available
    ServiceUnavailable,
    /// Speech-to-text provider does not support the received audio format
    InvalidAudio,
    /// Text extraction failed, no text recognized
    NoTextRecognized,
    /// Unexpected error during intent recognition
    IntentFailed,
    /// Unexpected error during text-to-speech
    TtsFailed,
    Timeout,
    UnexpectedError,
}

/// Available entity definition provided by an integration.
///
/// The `entity_type` value acts as a discriminator for the entity type, which defines the supported
/// features and options of an entity.
/// E.g., a simple `button` entity supports on / off, whereas a `cover` entity can be a simple window
/// cover with only the ability to open / close, or a venetian blind with position and tilting
/// features.
///
/// See entity documentation for more information about the individual entity features and options.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AvailableIntgEntity {
    /// Unique entity identifier within the integration device.
    #[validate(length(
        min = 1,
        max = 36,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 36)"
    ))]
    #[validate(regex(path = "*REGEX_ID_CHARS"))]
    pub entity_id: String,
    /// Optional associated device, only if the integration driver supports multiple devices.
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    #[validate(regex(path = "*REGEX_ID_CHARS"))]
    pub device_id: Option<String>,
    /// Discriminator value for the concrete entity device type.
    pub entity_type: EntityType,
    /// Optional device type. The UI can use this to represent the entity with a different
    /// icon, behaviour, etc. See entity documentation for available device classes.
    #[validate(length(max = 20, message = "Invalid length (max = 20)"))]
    pub device_class: Option<String>,
    /// Display-name of the entity in the UI.
    /// An English text with key `en` should always be provided as a fallback option. Otherwise, it's
    /// not guaranteed which text will be displayed if the user's selected language is not provided.
    pub name: HashMap<String, String>,
    /// Optional entity icon. Default entity type icon is used if not provided.
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    #[validate(regex(path = "*REGEX_ICON_ID"))]
    pub icon: Option<String>,
    /// Supported features of the entity.
    /// See entity-specific feature enums and the entity documentation for available features.
    pub features: Option<Vec<String>>,
    /// Optional area if supported by the integration. E.g. `Living room`.
    #[validate(length(max = 50, message = "Invalid length (max = 50)"))]
    pub area: Option<String>,
    /// Optional entity options. See entity documentation for available options.
    pub options: Option<serde_json::Map<String, Value>>,
    /// Optional entity attributes.
    ///
    /// If the dynamic entity attributes like `state` are already available, they should be included
    /// for the entity setup process. Otherwise, defaults are used (e.g. `state=UNKNOWN`).
    /// See entity documentation for available attributes.
    pub attributes: Option<serde_json::Map<String, Value>>,
}

impl AvailableIntgEntity {
    pub fn with_options<T>(mut self, options: T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        if let Value::Object(map) = serde_json::to_value(options)? {
            self.options = Some(map);
            Ok(self)
        } else {
            Err(serde_json::Error::custom("Invalid options object"))
        }
    }

    pub fn with_attributes<T>(mut self, attributes: T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        if let Value::Object(map) = serde_json::to_value(attributes)? {
            self.attributes = Some(map);
            Ok(self)
        } else {
            Err(serde_json::Error::custom("Invalid attributes object"))
        }
    }
}

/// Integration-API remote entity option fields.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::RemoteOptionField]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgRemoteOptionField {
    /// Supported commands of the remote.
    SimpleCommands,
    /// Optional command mapping for the physical buttons.
    ButtonMapping,
    /// Optional user interface definition for the supported commands.
    UserInterface,
}

/// Integration-API remote features.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::RemoteFeature]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgRemoteFeature {
    OnOff,
    Toggle,
    SendCmd,
    StopSend,
}

/// Integration-API remote entity commands.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::RemoteCommand]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgRemoteCommand {
    On,
    Off,
    Toggle,
    SendCmd,
    StopSend,
    SendCmdSequence,
}

/// Integration-API IR-emitter features.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::IrEmitterFeature]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgIrEmitterFeature {
    SendIr,
}

/// Integration-API IR-emitter entity commands.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::IrEmitterCommand]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgIrEmitterCommand {
    SendIr,
    StopIr,
}

/// Integration-API IR-emitter entity option fields.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::IrEmitterOptionField]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgIrEmitterOptionField {
    Ports,
    IrFormats,
}

/// Integration-API Voice Assistant entity commands.
///
/// Attention: only valid in the Integration-API data model. See [crate::core::VoiceAssistantCommand]
/// for the Core-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IntgVoiceAssistantCommand {
    /// Request to start sending a voice command audio stream to the integration.
    ///
    /// After confirming the command, the audio stream is started and transmitted as binary
    /// WebSocket messages in protocol buffer format.
    VoiceStart,
}
