// Copyright (c) 2023 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Shared integration models

use crate::model::settings::{ConfirmationPage, SettingsPage};
use serde::{Deserialize, Serialize};
use strum_macros::*;

#[derive(
    Debug, Clone, Copy, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SetupChangeEventType {
    /// Setup started.
    Start,
    /// Setup in progress. See `state` enum for current setup state.
    Setup,
    /// Setup finished, either with: `state: OK` for successful setup, or `state: ERROR` if setup
    /// didn't complete successfully.
    Stop,
}

#[derive(
    Debug, Clone, Copy, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationSetupState {
    /// Internal state while preparing setup.
    New,
    /// Setup in progress.
    Setup,
    /// Setup flow is waiting for user input. See `require_user_action` for requested input.
    WaitUserAction,
    /// Setup finished successfully.
    Ok,
    /// Setup error.
    Error,
}

// TODO enhance IntegrationSetupError enum?
/// More detailed error reason for `state: ERROR` condition.
#[derive(
    Debug, Clone, Copy, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationSetupError {
    None,
    NotFound,
    ConnectionRefused,
    AuthorizationError,
    Timeout,
    Other,
}

/// If set, the setup process waits for the specified user action.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequireUserAction {
    Input(SettingsPage),
    Confirmation(ConfirmationPage),
}
