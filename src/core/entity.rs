// Copyright (c) 2024 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Core-API related entity data structures.

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, VariantNames};

/// Core-API remote entity option fields.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgRemoteOptionField]
/// for the Integration-API data model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}

/// Core-API remote entity commands.
///
/// Attention: only valid in the Core-API data model. See [crate::intg::IntgRemoteCommand]
/// for the Integration-API data model.
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
    SendCmd,
    SendCmdSequence,
}
