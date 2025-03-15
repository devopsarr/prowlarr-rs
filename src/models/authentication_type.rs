/*
 * Prowlarr
 *
 * Prowlarr API docs
 *
 * The version of the OpenAPI document: v1.31.2.4975
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticationType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "forms")]
    Forms,
    #[serde(rename = "external")]
    External,

}

impl std::fmt::Display for AuthenticationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Basic => write!(f, "basic"),
            Self::Forms => write!(f, "forms"),
            Self::External => write!(f, "external"),
        }
    }
}

impl Default for AuthenticationType {
    fn default() -> AuthenticationType {
        Self::None
    }
}

