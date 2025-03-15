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
pub struct IndexerCapabilityResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "limitsMax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limits_max: Option<Option<i32>>,
    #[serde(rename = "limitsDefault", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limits_default: Option<Option<i32>>,
    #[serde(rename = "categories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Option<Vec<models::IndexerCategory>>>,
    #[serde(rename = "supportsRawSearch", skip_serializing_if = "Option::is_none")]
    pub supports_raw_search: Option<bool>,
    #[serde(rename = "searchParams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub search_params: Option<Option<Vec<models::SearchParam>>>,
    #[serde(rename = "tvSearchParams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tv_search_params: Option<Option<Vec<models::TvSearchParam>>>,
    #[serde(rename = "movieSearchParams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movie_search_params: Option<Option<Vec<models::MovieSearchParam>>>,
    #[serde(rename = "musicSearchParams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub music_search_params: Option<Option<Vec<models::MusicSearchParam>>>,
    #[serde(rename = "bookSearchParams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub book_search_params: Option<Option<Vec<models::BookSearchParam>>>,
}

impl IndexerCapabilityResource {
    pub fn new() -> IndexerCapabilityResource {
        IndexerCapabilityResource {
            id: None,
            limits_max: None,
            limits_default: None,
            categories: None,
            supports_raw_search: None,
            search_params: None,
            tv_search_params: None,
            movie_search_params: None,
            music_search_params: None,
            book_search_params: None,
        }
    }
}

