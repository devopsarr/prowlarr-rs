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
pub struct IndexerStatsResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "indexers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub indexers: Option<Option<Vec<models::IndexerStatistics>>>,
    #[serde(rename = "userAgents", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_agents: Option<Option<Vec<models::UserAgentStatistics>>>,
    #[serde(rename = "hosts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Option<Vec<models::HostStatistics>>>,
}

impl IndexerStatsResource {
    pub fn new() -> IndexerStatsResource {
        IndexerStatsResource {
            id: None,
            indexers: None,
            user_agents: None,
            hosts: None,
        }
    }
}

