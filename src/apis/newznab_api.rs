/*
 * Prowlarr
 *
 * Prowlarr API docs
 *
 * The version of the OpenAPI document: v1.31.2.4975
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_indexer_download`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIndexerDownloadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_indexer_newznab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIndexerNewznabError {
    UnknownValue(serde_json::Value),
}


pub async fn get_indexer_download(configuration: &configuration::Configuration, id: i32, link: Option<&str>, file: Option<&str>) -> Result<(), Error<GetIndexerDownloadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_link = link;
    let p_file = file;

    let uri_str = format!("{}/api/v1/indexer/{id}/download", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_link {
        req_builder = req_builder.query(&[("link", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_file {
        req_builder = req_builder.query(&[("file", &param_value.to_string())]);
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
        let entity: Option<GetIndexerDownloadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_indexer_newznab(configuration: &configuration::Configuration, id: i32, t: Option<&str>, q: Option<&str>, cat: Option<&str>, imdbid: Option<&str>, tmdbid: Option<i32>, extended: Option<&str>, limit: Option<i32>, offset: Option<i32>, minage: Option<i32>, maxage: Option<i32>, minsize: Option<i64>, maxsize: Option<i64>, rid: Option<i32>, tvmazeid: Option<i32>, traktid: Option<i32>, tvdbid: Option<i32>, doubanid: Option<i32>, season: Option<i32>, ep: Option<&str>, album: Option<&str>, artist: Option<&str>, label: Option<&str>, track: Option<&str>, year: Option<i32>, genre: Option<&str>, author: Option<&str>, title: Option<&str>, publisher: Option<&str>, configured: Option<&str>, source: Option<&str>, host: Option<&str>, server: Option<&str>) -> Result<(), Error<GetIndexerNewznabError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_t = t;
    let p_q = q;
    let p_cat = cat;
    let p_imdbid = imdbid;
    let p_tmdbid = tmdbid;
    let p_extended = extended;
    let p_limit = limit;
    let p_offset = offset;
    let p_minage = minage;
    let p_maxage = maxage;
    let p_minsize = minsize;
    let p_maxsize = maxsize;
    let p_rid = rid;
    let p_tvmazeid = tvmazeid;
    let p_traktid = traktid;
    let p_tvdbid = tvdbid;
    let p_doubanid = doubanid;
    let p_season = season;
    let p_ep = ep;
    let p_album = album;
    let p_artist = artist;
    let p_label = label;
    let p_track = track;
    let p_year = year;
    let p_genre = genre;
    let p_author = author;
    let p_title = title;
    let p_publisher = publisher;
    let p_configured = configured;
    let p_source = source;
    let p_host = host;
    let p_server = server;

    let uri_str = format!("{}/api/v1/indexer/{id}/newznab", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_t {
        req_builder = req_builder.query(&[("t", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_q {
        req_builder = req_builder.query(&[("q", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_cat {
        req_builder = req_builder.query(&[("cat", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_imdbid {
        req_builder = req_builder.query(&[("imdbid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_tmdbid {
        req_builder = req_builder.query(&[("tmdbid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_extended {
        req_builder = req_builder.query(&[("extended", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_minage {
        req_builder = req_builder.query(&[("minage", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_maxage {
        req_builder = req_builder.query(&[("maxage", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_minsize {
        req_builder = req_builder.query(&[("minsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_maxsize {
        req_builder = req_builder.query(&[("maxsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_rid {
        req_builder = req_builder.query(&[("rid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_tvmazeid {
        req_builder = req_builder.query(&[("tvmazeid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_traktid {
        req_builder = req_builder.query(&[("traktid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_tvdbid {
        req_builder = req_builder.query(&[("tvdbid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_doubanid {
        req_builder = req_builder.query(&[("doubanid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_season {
        req_builder = req_builder.query(&[("season", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ep {
        req_builder = req_builder.query(&[("ep", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_album {
        req_builder = req_builder.query(&[("album", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_artist {
        req_builder = req_builder.query(&[("artist", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_label {
        req_builder = req_builder.query(&[("label", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_track {
        req_builder = req_builder.query(&[("track", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_year {
        req_builder = req_builder.query(&[("year", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_genre {
        req_builder = req_builder.query(&[("genre", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_author {
        req_builder = req_builder.query(&[("author", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_title {
        req_builder = req_builder.query(&[("title", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_publisher {
        req_builder = req_builder.query(&[("publisher", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_configured {
        req_builder = req_builder.query(&[("configured", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_source {
        req_builder = req_builder.query(&[("source", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_host {
        req_builder = req_builder.query(&[("host", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_server {
        req_builder = req_builder.query(&[("server", &param_value.to_string())]);
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
        let entity: Option<GetIndexerNewznabError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

