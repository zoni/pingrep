pub mod client;

use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
//use std::fmt;
//use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

pub const PINBOARD_API_URL: &str = "https://api.pinboard.in/v1/";

#[non_exhaustive]
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("HTTP request error: {source}"))]
    HttpError {
        source: reqwest::Error,
    },
    JsonDecodeError {
        source: serde_json::Error,
    },
}

/// A collection of [bookmarks][`Bookmark`]..
pub struct BookmarkCollection {
    /// The user whose bookmarks are contained in this collection.
    pub user: String,
    /// The bookmarks in this collection.
    pub bookmarks: Vec<Bookmark>,
    /// The time the user's bookmarks were last updated.
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    /// The URL of the bookmark.
    pub href: String,
    /// The title of the bookmark.
    #[serde(rename(deserialize = "description"))]
    pub title: String,
    /// Extended description of the bookmark.
    #[serde(rename(deserialize = "extended"))]
    pub description: String,
    /// The tags associated with the bookmark.
    #[serde(deserialize_with = "from_space_separated_string")]
    pub tags: Vec<String>,
    /// The time the bookmark was created.
    pub time: DateTime<Utc>,
    /// Whether the bookmark is marked as "shared".
    pub shared: String,
    /// Whether the bookmark is marked as "to read".
    pub toread: String,
    /// A unique ID for the bookmark
    pub meta: String,
    /// A hash of the bookmark.
    pub hash: String,
}

fn from_space_separated_string<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(' ')
        .filter(|&tag| tag != "")
        .map(|tag| tag.to_owned())
        .collect())
}

impl BookmarkCollection {
    pub fn from_string(json: String) -> Result<Vec<Bookmark>> {
        // TODO: Accept generic reader and use StreamDeserializer
        // (https://docs.rs/serde_json/latest/serde_json/struct.StreamDeserializer.html)
        let pins: Vec<Bookmark> = serde_json::from_str(&json).context(JsonDecodeSnafu)?;
        Ok(pins)
    }
}
