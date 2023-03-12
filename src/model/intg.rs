// Copyright (c) 2023 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Shared integration models

use crate::model::settings::{ConfirmationPage, SettingsPage};
use serde::{Deserialize, Serialize};
use strum_macros::*;

#[derive(Debug, Clone, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SetupChangeEventType {
    Start,
    Setup,
    Stop,
}

#[derive(Debug, Clone, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationSetupState {
    New,
    Setup,
    WaitUserAction,
    Ok,
    Error,
}

// TODO enhance IntegrationSetupError enum?
#[derive(Debug, Clone, AsRefStr, Display, EnumString, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequireUserAction {
    Input(SettingsPage),
    Confirmation(ConfirmationPage),
}
