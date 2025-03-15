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
pub struct TaskResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "taskName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub task_name: Option<Option<String>>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "lastExecution", skip_serializing_if = "Option::is_none")]
    pub last_execution: Option<String>,
    #[serde(rename = "lastStartTime", skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<String>,
    #[serde(rename = "nextExecution", skip_serializing_if = "Option::is_none")]
    pub next_execution: Option<String>,
    #[serde(rename = "lastDuration", skip_serializing_if = "Option::is_none")]
    pub last_duration: Option<String>,
}

impl TaskResource {
    pub fn new() -> TaskResource {
        TaskResource {
            id: None,
            name: None,
            task_name: None,
            interval: None,
            last_execution: None,
            last_start_time: None,
            next_execution: None,
            last_duration: None,
        }
    }
}

