/*
 * Prowlarr
 *
 * Prowlarr API docs
 *
 * The version of the OpenAPI document: v1.30.2.4939
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrivacyLevel {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "apiKey")]
    ApiKey,
    #[serde(rename = "userName")]
    UserName,

}

impl std::fmt::Display for PrivacyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Password => write!(f, "password"),
            Self::ApiKey => write!(f, "apiKey"),
            Self::UserName => write!(f, "userName"),
        }
    }
}

impl Default for PrivacyLevel {
    fn default() -> PrivacyLevel {
        Self::Normal
    }
}

