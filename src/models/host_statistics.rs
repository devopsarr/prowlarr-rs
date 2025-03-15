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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostStatistics {
    #[serde(rename = "host", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub host: Option<Option<String>>,
    #[serde(rename = "numberOfQueries", skip_serializing_if = "Option::is_none")]
    pub number_of_queries: Option<i32>,
    #[serde(rename = "numberOfGrabs", skip_serializing_if = "Option::is_none")]
    pub number_of_grabs: Option<i32>,
}

impl HostStatistics {
    pub fn new() -> HostStatistics {
        HostStatistics {
            host: None,
            number_of_queries: None,
            number_of_grabs: None,
        }
    }
}

