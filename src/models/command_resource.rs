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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "commandName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub command_name: Option<Option<String>>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<models::Command>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<models::CommandPriority>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::CommandStatus>,
    #[serde(rename = "queued", skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    #[serde(rename = "started", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started: Option<Option<String>>,
    #[serde(rename = "ended", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ended: Option<Option<String>>,
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<String>>,
    #[serde(rename = "exception", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exception: Option<Option<String>>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<models::CommandTrigger>,
    #[serde(rename = "clientUserAgent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_user_agent: Option<Option<String>>,
    #[serde(rename = "stateChangeTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state_change_time: Option<Option<String>>,
    #[serde(rename = "sendUpdatesToClient", skip_serializing_if = "Option::is_none")]
    pub send_updates_to_client: Option<bool>,
    #[serde(rename = "updateScheduledTask", skip_serializing_if = "Option::is_none")]
    pub update_scheduled_task: Option<bool>,
    #[serde(rename = "lastExecutionTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<Option<String>>,
}

impl CommandResource {
    pub fn new() -> CommandResource {
        CommandResource {
            id: None,
            name: None,
            command_name: None,
            message: None,
            body: None,
            priority: None,
            status: None,
            queued: None,
            started: None,
            ended: None,
            duration: None,
            exception: None,
            trigger: None,
            client_user_agent: None,
            state_change_time: None,
            send_updates_to_client: None,
            update_scheduled_task: None,
            last_execution_time: None,
        }
    }
}

