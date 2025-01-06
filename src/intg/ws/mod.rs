// Copyright (c) 2022 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration API specific WebSocket messages.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use strum_macros::*;
use url::Url;
use validator::Validate;

use crate::intg::{AvailableIntgEntity, DeviceState, IntegrationVersion};
use crate::model::Oauth2Token;
use crate::EntityType;

/// Remote Two initiated request messages for the integration driver.
///
/// The corresponding response message name is set with the strum message macro
///
/// # Examples
///
/// Deserialize from string:
/// ```
/// use std::str::FromStr;
/// use uc_api::intg::ws::R2Request;
/// assert_eq!(Ok(R2Request::SubscribeEvents), R2Request::from_str("subscribe_events"));
/// ```
// TODO refactor response message relationship? Use a typed DriverResponse variant as enum data.
//      --> verify Serde if data can be ignored while (de)serializing. This was the reason for using
//          the simple strum message approach...
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumMessage, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum R2Request {
    #[strum(message = "driver_version")]
    GetDriverVersion,
    // returns an event instead of a response message
    GetDeviceState,
    #[strum(message = "available_entities")]
    GetAvailableEntities,
    #[strum(message = "result")]
    SubscribeEvents,
    #[strum(message = "result")]
    UnsubscribeEvents,
    #[strum(message = "entity_states")]
    GetEntityStates,
    #[strum(message = "result")]
    EntityCommand,
    #[strum(message = "driver_metadata")]
    GetDriverMetadata,
    #[strum(message = "result")]
    SetupDriver,
    #[strum(message = "result")]
    SetDriverUserData,
}

/// Remote Two response messages for the integration driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum R2Response {
    Version,
    SupportedEntityTypes,
    ConfiguredEntities,
    LocalizationCfg,
    RuntimeInfo,
    Oauth2AuthUrl,
    Oauth2Token,
}

/// Integration specific events emitted from Remote Two
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum R2Event {
    Connect,
    Disconnect,
    EnterStandby,
    ExitStandby,
    AbortDriverSetup,
    Oauth2Authorization,
}

/// Integration driver response messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum DriverResponse {
    Result,
    DriverVersion,
    AvailableEntities,
    EntityStates,
    DriverMetadata,
}

/// Events emitted from the integration driver
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum DriverEvent {
    AuthRequired,
    DeviceState,
    EntityChange,
    EntityAvailable,
    EntityRemoved,
    DriverSetupChange,
}

/// Request messages initiated from the Remote to the integration driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(AsRefStr, Display, EnumString, VariantNames)] // strum_macros
#[strum(serialize_all = "snake_case")]
pub enum DriverRequest {
    GetVersion,
    GetSupportedEntityTypes,
    GetConfiguredEntities,
    GetLocalizationCfg,
    GetRuntimeInfo,
    GenerateOauth2AuthUrl,
    CreateOauth2Cfg,
    GetOauth2Token,
    DeleteOauth2Token,
}

/// Payload data of a `driver_version` response message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DriverVersionMsgData {
    /// Only required for multi-device integrations.
    pub name: Option<String>,
    pub version: Option<IntegrationVersion>,
}

/// Payload data of a `device_state` event message in `msg_data` property.  
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceStateMsgData {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub state: DeviceState,
}

/// Payload data of `entity_available` event message in `msg_data` property.
///
/// This is an optional event and not yet implemented in the core.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityAvailableMsgData {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub features: Option<Vec<String>>,
    pub name: HashMap<String, String>,
    pub area: Option<String>,
}

/// Payload data of `entity_removed` event message in `msg_data` property.
///  
/// This is an optional event and not yet implemented in the core.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityRemovedMsgData {
    /// Only required for multi-device integrations.
    pub device_id: Option<String>,
    pub entity_type: EntityType,
    pub entity_id: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AvailableEntitiesFilter {
    pub device_id: Option<String>,
    pub entity_type: Option<EntityType>,
}

/// Payload data of `available_entities` response message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AvailableEntitiesMsgData {
    pub filter: Option<AvailableEntitiesFilter>,
    #[validate]
    pub available_entities: Vec<AvailableIntgEntity>,
}

/// Payload data of `runtime_info` response message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct RuntimeInfoMsgData {
    pub driver_id: String,
    pub intg_ids: Vec<String>,
    pub log_id: Option<String>,
}

/// Payload data of `generate_oauth2_auth_url` request message in `msg_data` property.
///
/// Request an OAuth2 authorization URL for the user.
///
/// A new unique authorization URL is created, which may only be used once. Only one active authorization URL per
/// integration is allowed. Old URLs will become invalid if multiple calls are made.
///
/// Once the user has authorized the integration driver by the authorization server, the `oauth2_authorization`
/// event is emitted.
///
/// The `client_data` dictionary can be used to encode driver specific data into the URL, which will be returned
/// in the `oauth2_authorization` event. This allows to link the authorization event to internal driver states,
/// e.g. a setup flow step.
///
/// The following data fields are automatically injected by the core and overwritten if set by an integration:
/// - `intg`: set to the integration driver_id.
/// - `acc`: set to the default integration name.
/// - `dev`: set to the device address, used for the authentication callback URL.
///
/// ℹ️️ implemented in firmware 2.2.3.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct GenerateOauth2AuthUrlMsgData {
    /// Additional key-value pairs which should be encoded into the `state` query parameter of the
    /// authorization request.
    pub client_data: HashMap<String, String>,
}

/// Payload data of `oauth2_auth_url` response message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Oauth2AuthUrlMsgData {
    pub auth_url: Url,
}

/// Payload data of `create_oauth2_cfg` request message in `msg_data` property.
///
/// Create an OAuth2 configuration entry in the Core.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateOauth2CfgMsgData {
    /// Token identifier to use for the OAuth2 token
    #[validate(length(min = 1, max = 512, message = "Invalid length (min = 1, max = 512)"))]
    pub token_id: String,
    /// Friendly name of the OAuth2 token
    #[validate(length(min = 1, max = 50, message = "Invalid length (min = 1, max = 50)"))]
    pub name: String,
    /// The OAuth2 token as received in the oauth2_authorization event.
    pub token: Oauth2Token,
}

/// Payload data of `get_oauth2_token` request message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct GetOauth2TokenMsgData {
    pub token_id: String,
    /// Force a token refresh, no matter if the current token is still valid or not.
    pub force_refresh: Option<bool>,
}

/// Payload data of `oauth2_token` response message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Oauth2TokenMsgData {
    pub token_id: String,
    pub token: Oauth2Token,
}

/// Payload data of `delete_oauth2_token` request message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct DeleteOauth2TokenMsgData {
    pub token_id: String,
}

/// Payload data of `oauth2_authorization` event message in `msg_data` property.
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Oauth2AuthorizationMsgData {
    /// Provided key-value pairs in the authorization request URL.
    pub client_data: HashMap<String, String>,
    /// Set if the authorization failed.
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    /// Only set if authorization has been successful.
    pub token: Option<Oauth2Token>,
}

impl Oauth2AuthorizationMsgData {
    pub fn ok(client_data: HashMap<String, String>, token: Oauth2Token) -> Self {
        Self {
            client_data,
            error_code: None,
            error_description: None,
            token: Some(token),
        }
    }

    pub fn error<S: ToString, T: ToString>(
        client_data: HashMap<String, String>,
        code: S,
        description: Option<T>,
    ) -> Self {
        Self {
            client_data,
            error_code: Some(code.to_string()),
            error_description: description.map(|d| d.to_string()),
            token: None,
        }
    }
}
