// Copyright (c) 2024 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Core-API related entity data structures.

use crate::{VoiceAssistantFeature, VoiceAssistantProfile};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use strum_macros::{AsRefStr, Display, EnumString, VariantNames};

/// Core-API remote entity option fields.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgRemoteOptionField]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteOptionField {
    Editable,
    /// List of command identifiers which can be used in the `send` command.
    ///
    /// For an IR remote these are the IR command keys of the associated or learned IR dataset.
    SimpleCommands,
    ButtonMapping,
    UserInterface,
}

/// Core-API remote features.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgRemoteFeature]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum RemoteFeature {
    OnOff,
    Toggle,
    /// Send IR commands
    Send,
    /// Stop sending a repeated IR command
    StopSend,
    /// Send arbitrary commands
    SendCmd,
    /// Send an input key
    SendKey,
}

/// Core-API remote entity commands.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgRemoteCommand]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    SendCmd,
    SendCmdSequence,
    SendKey,
}

/// Core-API IR-emitter features.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgIrEmitterFeature]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IrEmitterFeature {
    SendIr,
}

/// Core-API IR-emitter entity commands.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgIrEmitterCommand]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IrEmitterCommand {
    SendIr,
    StopIr,
}

/// Core-API IR-emitter entity option fields.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgIrEmitterOptionField]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum IrEmitterOptionField {
    Ports,
    IrFormats,
}

/// Core-API Voice Assistant entity commands.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgVoiceAssistantCommand]
/// for the Integration-API data model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum VoiceAssistantCommand {
    /// Request to start sending a voice command audio stream to the integration.
    ///
    /// The stream is sent to the integration after receiving the [crate::intg::AssistantEvent::Ready] event.
    VoiceStart,
    /// Stop the voice command audio stream.
    VoiceEnd,
}

/// Voice assistant definition.
///
/// This is a tailored representation of the voice_assistant entity, which can be used to display
/// voice assistant information to users.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceAssistant {
    pub entity_id: String,
    /// Display-name of the voice assistant in the UI.
    pub name: HashMap<String, String>,
    /// Optional icon identifier.
    pub icon: Option<String>,
    /// Entity state
    pub state: Option<String>,
    /// Default features supported by the voice assistant.
    /// If multiple profiles are supported, it should be the feature list of the preferred profile.
    pub features: Option<Vec<VoiceAssistantFeature>>,
    /// Optional profiles that can be used by starting a voice command.
    pub profiles: Option<Vec<VoiceAssistantProfile>>,
    /// Preferred profile specified by the integration.
    pub preferred_profile: Option<String>,
}
