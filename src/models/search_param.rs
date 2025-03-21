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
pub enum SearchParam {
    #[serde(rename = "q")]
    Q,

}

impl std::fmt::Display for SearchParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Q => write!(f, "q"),
        }
    }
}

impl Default for SearchParam {
    fn default() -> SearchParam {
        Self::Q
    }
}

