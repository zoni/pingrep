pub mod client;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

pub const PINBOARD_API_URL: &str = "https://api.pinboard.in/v1/";

#[non_exhaustive]
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("HTTP request error: {source}"))]
    HttpError {
        source: reqwest::Error,
    },
    JsonEncodeError {
        source: serde_json::Error,
    },
    JsonDecodeError {
        source: serde_json::Error,
    },
    #[snafu(display("Malformed token: {reason}"))]
    MalformedTokenError {
        reason: String,
    },
}

/// A collection of [bookmarks][`Bookmark`]..
#[derive(Serialize, Deserialize, Debug)]
pub struct BookmarkCollection {
    /// The time the user's bookmarks were last updated.
    pub last_updated: DateTime<Utc>,
    /// The user whose bookmarks are contained in this collection.
    pub user: String,
    /// The bookmarks in this collection.
    pub bookmarks: Vec<Bookmark>,
}

/// An individual pinboard bookmark.
#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    /// The URL of the bookmark.
    pub url: String,
    /// The title of the bookmark.
    pub title: String,
    /// Extended description of the bookmark.
    pub description: String,
    /// The tags associated with the bookmark.
    pub tags: Vec<String>,
    /// The time the bookmark was created.
    pub time: DateTime<Utc>,
    /// Whether the bookmark is marked as "shared".
    pub shared: bool,
    /// Whether the bookmark is marked as "to read".
    pub toread: bool,
    /// A unique ID for the bookmark
    pub meta: String,
    /// A hash of the bookmark.
    pub hash: String,
}

impl BookmarkCollection {
    /// Read a bookmark collection from the given reader.
    pub fn read<R>(reader: R) -> Result<BookmarkCollection>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).context(JsonDecodeSnafu {})
    }

    /// Write a bookmark collection to the given writer.
    pub fn write<W>(&self, writer: W) -> Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self).context(JsonEncodeSnafu {})
    }

    /// Create a bookmark collection from the pinboard API.
    pub fn from_api(client: &client::Client) -> Result<BookmarkCollection> {
        let user = client.user()?;
        let last_updated = client.last_update()?;
        let bookmarks = client.bookmarks()?;
        Ok(BookmarkCollection {
            user,
            bookmarks,
            last_updated,
        })
    }

    /// Find a bookmark by URL.
    pub fn find_by_url(&self, url: &str) -> Option<&Bookmark> {
        self.bookmarks.iter().find(|b| b.url == url)
    }
}
