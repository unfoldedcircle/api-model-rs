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

/// More detailed error reason for `state: ERROR` condition.
#[derive(
    Debug, Clone, Copy, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationSetupError {
    None,
    /// The driver could not find the device or service to set up.
    NotFound,
    /// The driver could not connect to the device or service.
    ConnectionRefused,
    /// The device or service rejected the credentials or the pairing.
    AuthorizationError,
    /// The driver timed out while communicating with the device or service.
    Timeout,
    /// The integration driver is not running, not connected, or its connection could not be
    /// re-established. Check the integration driver service and start the setup again.
    DriverUnavailable,
    /// The provided user data was rejected. When reported together with `state: WAIT_USER_ACTION`
    /// and a `require_user_action` page, the setup continues and the user can correct the input.
    InvalidInput,
    /// The setup was stopped by a client.
    Aborted,
    /// The driver refused the setup because an integration instance already exists and no
    /// reconfiguration was requested.
    AlreadyConfigured,
    /// The driver does not support the requested operation, e.g. reconfiguration.
    NotSupported,
    /// Any other error, see error_message if provided.
    Other,
}

/// If set, the setup process waits for the specified user action.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequireUserAction {
    Input(SettingsPage),
    Confirmation(ConfirmationPage),
}
