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
pub struct AppProfileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "enableRss", skip_serializing_if = "Option::is_none")]
    pub enable_rss: Option<bool>,
    #[serde(rename = "enableAutomaticSearch", skip_serializing_if = "Option::is_none")]
    pub enable_automatic_search: Option<bool>,
    #[serde(rename = "enableInteractiveSearch", skip_serializing_if = "Option::is_none")]
    pub enable_interactive_search: Option<bool>,
    #[serde(rename = "minimumSeeders", skip_serializing_if = "Option::is_none")]
    pub minimum_seeders: Option<i32>,
}

impl AppProfileResource {
    pub fn new() -> AppProfileResource {
        AppProfileResource {
            id: None,
            name: None,
            enable_rss: None,
            enable_automatic_search: None,
            enable_interactive_search: None,
            minimum_seeders: None,
        }
    }
}

