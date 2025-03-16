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


/// struct for typed errors of method [`get_indexer_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIndexerStatsError {
    UnknownValue(serde_json::Value),
}


pub async fn get_indexer_stats(configuration: &configuration::Configuration, start_date: Option<String>, end_date: Option<String>, indexers: Option<&str>, protocols: Option<&str>, tags: Option<&str>) -> Result<models::IndexerStatsResource, Error<GetIndexerStatsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_indexers = indexers;
    let p_protocols = protocols;
    let p_tags = tags;

    let uri_str = format!("{}/api/v1/indexerstats", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_indexers {
        req_builder = req_builder.query(&[("indexers", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_protocols {
        req_builder = req_builder.query(&[("protocols", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_tags {
        req_builder = req_builder.query(&[("tags", &param_value.to_string())]);
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
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::IndexerStatsResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::IndexerStatsResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIndexerStatsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

