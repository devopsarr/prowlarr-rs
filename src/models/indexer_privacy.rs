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
pub enum IndexerPrivacy {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "semiPrivate")]
    SemiPrivate,
    #[serde(rename = "private")]
    Private,

}

impl std::fmt::Display for IndexerPrivacy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::SemiPrivate => write!(f, "semiPrivate"),
            Self::Private => write!(f, "private"),
        }
    }
}

impl Default for IndexerPrivacy {
    fn default() -> IndexerPrivacy {
        Self::Public
    }
}

