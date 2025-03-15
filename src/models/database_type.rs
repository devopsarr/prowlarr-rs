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
pub enum DatabaseType {
    #[serde(rename = "sqLite")]
    SqLite,
    #[serde(rename = "postgreSQL")]
    PostgreSql,

}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SqLite => write!(f, "sqLite"),
            Self::PostgreSql => write!(f, "postgreSQL"),
        }
    }
}

impl Default for DatabaseType {
    fn default() -> DatabaseType {
        Self::SqLite
    }
}

