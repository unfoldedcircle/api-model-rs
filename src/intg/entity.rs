// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration API related entity data structures.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::{EntityType, REGEX_ID_CHARS};

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
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityCommand {
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub cmd_id: String,
    pub params: Option<serde_json::Map<String, Value>>,
}

/// Entity state change event.
///
/// Emitted when an attribute of an entity changes, e.g. is switched off. Either after an `entity_command` or if the
/// entity is updated manually through a user or an external system. This keeps the remote in sync with the real
/// state of the entity without the need of constant polling.
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityChange {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    /// Integration specific entity identifier.
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
    /// Unique entity identifier within the integration device.
    #[validate(length(
        min = 1,
        max = 36,
        code = "INVALID_LENGTH",
        message = "Invalid length (min = 1, max = 36)"
    ))]
    #[validate(regex(path = "REGEX_ID_CHARS"))]
    pub entity_id: String,
    /// Optional associated device, only if the integration driver supports multiple devices.
    #[validate(length(max = 36, message = "Invalid length (max = 36)"))]
    #[validate(regex(path = "REGEX_ID_CHARS"))]
    pub device_id: Option<String>,
    /// Discriminator value for the concrete entity device type.
    pub entity_type: EntityType,
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
