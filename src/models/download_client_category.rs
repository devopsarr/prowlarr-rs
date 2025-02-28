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
pub struct DownloadClientCategory {
    #[serde(rename = "clientCategory", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_category: Option<Option<String>>,
    #[serde(rename = "categories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Option<Vec<i32>>>,
}

impl DownloadClientCategory {
    pub fn new() -> DownloadClientCategory {
        DownloadClientCategory {
            client_category: None,
            categories: None,
        }
    }
}

