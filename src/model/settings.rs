// Copyright (c) 2022 Unfolded Circle ApS and/or its affiliates. All rights reserved. Use is subject to license terms.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use validator::Validate;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct ConfirmationPage {
    pub title: HashMap<String, String>,
    pub message1: Option<HashMap<String, String>>,
    pub image: Option<String>,
    pub message2: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct SettingsPage {
    pub title: HashMap<String, String>,
    #[validate]
    pub settings: Vec<Setting>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Setting {
    #[validate(length(min = 1, max = 50))]
    pub id: String,
    pub label: HashMap<String, String>,
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

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Number {
    pub value: IntOrFloat,
    pub min: Option<IntOrFloat>,
    pub max: Option<IntOrFloat>,
    pub steps: Option<i32>,
    pub decimals: Option<u8>,
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

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Text {
    pub value: Option<String>,
    pub regex: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Textarea {
    pub value: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Password {
    pub value: Option<String>,
    pub regex: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Checkbox {
    pub value: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Dropdown {
    pub value: Option<String>,
    #[validate]
    pub items: Vec<DropdownItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct DropdownItem {
    #[validate(length(min = 1, max = 50))]
    pub id: String,
    pub label: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Label {
    /// Static text to display next to the label
    pub value: HashMap<String, String>,
}
