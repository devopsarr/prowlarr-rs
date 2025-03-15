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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TvSearchParam {
    #[serde(rename = "q")]
    Q,
    #[serde(rename = "season")]
    Season,
    #[serde(rename = "ep")]
    Ep,
    #[serde(rename = "imdbId")]
    ImdbId,
    #[serde(rename = "tvdbId")]
    TvdbId,
    #[serde(rename = "rId")]
    RId,
    #[serde(rename = "tvMazeId")]
    TvMazeId,
    #[serde(rename = "traktId")]
    TraktId,
    #[serde(rename = "tmdbId")]
    TmdbId,
    #[serde(rename = "doubanId")]
    DoubanId,
    #[serde(rename = "genre")]
    Genre,
    #[serde(rename = "year")]
    Year,

}

impl std::fmt::Display for TvSearchParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Q => write!(f, "q"),
            Self::Season => write!(f, "season"),
            Self::Ep => write!(f, "ep"),
            Self::ImdbId => write!(f, "imdbId"),
            Self::TvdbId => write!(f, "tvdbId"),
            Self::RId => write!(f, "rId"),
            Self::TvMazeId => write!(f, "tvMazeId"),
            Self::TraktId => write!(f, "traktId"),
            Self::TmdbId => write!(f, "tmdbId"),
            Self::DoubanId => write!(f, "doubanId"),
            Self::Genre => write!(f, "genre"),
            Self::Year => write!(f, "year"),
        }
    }
}

impl Default for TvSearchParam {
    fn default() -> TvSearchParam {
        Self::Q
    }
}

