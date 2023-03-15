// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use validator::Validate;

/// Confirmation screen, e.g. to agree with something when setting up an integration driver.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct ConfirmationPage {
    /// Language specific page title.
    pub title: HashMap<String, String>,
    /// Language specific message to display between title and image (if supplied).
    ///
    /// Supports Markdown formatting.
    pub message1: Option<HashMap<String, String>>,
    /// Optional base64-encoded image (png or jpg).
    pub image: Option<String>,
    /// Language specific Message to display below message1 or image (if supplied).
    ///
    /// Supports Markdown formatting.
    pub message2: Option<HashMap<String, String>>,
}

/// Settings definition page, e.g. to configure an integration driver.
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct SettingsPage {
    /// Language specific settings page title.
    pub title: HashMap<String, String>,
    /// One or multiple input field definitions, with optional pre-set values.
    #[validate]
    pub settings: Vec<Setting>,
}

/// An input setting is of a specific type defined in `field.type` which defines how it is presented to the user.
///
/// Inspired by the [Homey SDK settings](https://apps.developer.homey.app/the-basics/devices/settings) concept.
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Setting {
    /// Unique identifier of the setting to be returned with the entered value.
    #[validate(length(min = 1, max = 50))]
    pub id: String,
    /// Language specific settings label.
    pub label: HashMap<String, String>,
    /// Input field or text information.
    pub field: Field,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Number(Number),
    Text(Text),
    Textarea(Textarea),
    Password(Password),
    Checkbox(Checkbox),
    Dropdown(Dropdown),
    Label(Label),
}

/// Number input with optional `min`, `max`, `steps` and `decimals` properties.
///
/// The default value must be specified in `value`. An optional unit of the number setting can be
/// specified in `units`, which will be displayed next to the input field.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Number {
    /// Default value for input field.
    pub value: IntOrFloat,
    /// Optional validation: minimum allowed value (inclusive).
    pub min: Option<IntOrFloat>,
    /// Optional validation: maximum allowed value (inclusive).
    pub max: Option<IntOrFloat>,
    /// Optional validation: allowed step increment between values. Might also be used in the UI for input helpers.
    pub steps: Option<i32>,
    /// Number of decimal places. None or 0 = integer value.
    pub decimals: Option<u8>,
    /// Language specific unit text. Displayed following the input field.
    pub unit: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntOrFloat {
    Int(i32),
    Float(f32),
}

impl From<i32> for IntOrFloat {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for IntOrFloat {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<IntOrFloat> for i32 {
    fn from(value: IntOrFloat) -> Self {
        match value {
            IntOrFloat::Int(v) => v,
            IntOrFloat::Float(v) => v.round() as i32,
        }
    }
}

impl From<IntOrFloat> for f32 {
    fn from(value: IntOrFloat) -> Self {
        match value {
            IntOrFloat::Int(v) => v as f32,
            IntOrFloat::Float(v) => v,
        }
    }
}

/// Single line of text input.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Text {
    /// Optional default value.
    pub value: Option<String>,
    /// Optional regex validation pattern for the input value.
    pub regex: Option<String>,
}

/// Multi-line text input, e.g. for providing a description.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Textarea {
    /// Optional default value.
    pub value: Option<String>,
}

/// Password or pin entry field with the input text hidden from the user.
///
/// Otherwise the same as text input.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Password {
    /// Optional default value.
    pub value: Option<String>,
    /// Optional regex validation pattern for the input value.
    pub regex: Option<String>,
}

/// Checkbox setting with `true` / `false` values.
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Checkbox {
    /// Initial setting.
    pub value: bool,
}

/// Dropdown setting to pick a single value from a list. All values must be strings.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Dropdown {
    /// Pre-selected dropdown id.
    pub value: Option<String>,
    #[validate]
    pub items: Vec<DropdownItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct DropdownItem {
    /// Selection identifier.
    #[validate(length(min = 1, max = 50))]
    pub id: String,
    /// Language specific text.
    pub label: HashMap<String, String>,
}

/// Additional read-only text for information purpose between other settings. Supports Markdown formatting.
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Label {
    /// Static text to display next to the label
    pub value: HashMap<String, String>,
}
