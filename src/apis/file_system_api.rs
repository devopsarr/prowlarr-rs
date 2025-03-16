/*
 * Prowlarr
 *
 * Prowlarr API docs
 *
 * The version of the OpenAPI document: v1.32.2.4987
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_file_system`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileSystemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_file_system_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileSystemTypeError {
    UnknownValue(serde_json::Value),
}


pub async fn get_file_system(configuration: &configuration::Configuration, path: Option<&str>, include_files: Option<bool>, allow_folders_without_trailing_slashes: Option<bool>) -> Result<(), Error<GetFileSystemError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path = path;
    let p_include_files = include_files;
    let p_allow_folders_without_trailing_slashes = allow_folders_without_trailing_slashes;

    let uri_str = format!("{}/api/v1/filesystem", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_path {
        req_builder = req_builder.query(&[("path", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_files {
        req_builder = req_builder.query(&[("includeFiles", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_allow_folders_without_trailing_slashes {
        req_builder = req_builder.query(&[("allowFoldersWithoutTrailingSlashes", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFileSystemError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_file_system_type(configuration: &configuration::Configuration, path: Option<&str>) -> Result<(), Error<GetFileSystemTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path = path;

    let uri_str = format!("{}/api/v1/filesystem/type", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_path {
        req_builder = req_builder.query(&[("path", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFileSystemTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

