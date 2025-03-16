/*
 * Prowlarr
 *
 * Prowlarr API docs
 *
 * The version of the OpenAPI document: v1.32.2.4987
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Option<Vec<models::Field>>>,
    #[serde(rename = "implementationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation_name: Option<Option<String>>,
    #[serde(rename = "implementation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<Option<String>>,
    #[serde(rename = "configContract", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config_contract: Option<Option<String>>,
    #[serde(rename = "infoLink", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_link: Option<Option<String>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::ProviderMessage>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "presets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presets: Option<Option<Vec<models::NotificationResource>>>,
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    #[serde(rename = "onGrab", skip_serializing_if = "Option::is_none")]
    pub on_grab: Option<bool>,
    #[serde(rename = "onHealthIssue", skip_serializing_if = "Option::is_none")]
    pub on_health_issue: Option<bool>,
    #[serde(rename = "onHealthRestored", skip_serializing_if = "Option::is_none")]
    pub on_health_restored: Option<bool>,
    #[serde(rename = "onApplicationUpdate", skip_serializing_if = "Option::is_none")]
    pub on_application_update: Option<bool>,
    #[serde(rename = "supportsOnGrab", skip_serializing_if = "Option::is_none")]
    pub supports_on_grab: Option<bool>,
    #[serde(rename = "includeManualGrabs", skip_serializing_if = "Option::is_none")]
    pub include_manual_grabs: Option<bool>,
    #[serde(rename = "supportsOnHealthIssue", skip_serializing_if = "Option::is_none")]
    pub supports_on_health_issue: Option<bool>,
    #[serde(rename = "supportsOnHealthRestored", skip_serializing_if = "Option::is_none")]
    pub supports_on_health_restored: Option<bool>,
    #[serde(rename = "includeHealthWarnings", skip_serializing_if = "Option::is_none")]
    pub include_health_warnings: Option<bool>,
    #[serde(rename = "supportsOnApplicationUpdate", skip_serializing_if = "Option::is_none")]
    pub supports_on_application_update: Option<bool>,
    #[serde(rename = "testCommand", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub test_command: Option<Option<String>>,
}

impl NotificationResource {
    pub fn new() -> NotificationResource {
        NotificationResource {
            id: None,
            name: None,
            fields: None,
            implementation_name: None,
            implementation: None,
            config_contract: None,
            info_link: None,
            message: None,
            tags: None,
            presets: None,
            link: None,
            on_grab: None,
            on_health_issue: None,
            on_health_restored: None,
            on_application_update: None,
            supports_on_grab: None,
            include_manual_grabs: None,
            supports_on_health_issue: None,
            supports_on_health_restored: None,
            include_health_warnings: None,
            supports_on_application_update: None,
            test_command: None,
        }
    }
}

