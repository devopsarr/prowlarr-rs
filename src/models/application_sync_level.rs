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
pub enum ApplicationSyncLevel {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "addOnly")]
    AddOnly,
    #[serde(rename = "fullSync")]
    FullSync,

}

impl std::fmt::Display for ApplicationSyncLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Disabled => write!(f, "disabled"),
            Self::AddOnly => write!(f, "addOnly"),
            Self::FullSync => write!(f, "fullSync"),
        }
    }
}

impl Default for ApplicationSyncLevel {
    fn default() -> ApplicationSyncLevel {
        Self::Disabled
    }
}

