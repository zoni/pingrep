use super::{HttpSnafu, MalformedTokenSnafu, Result};
use chrono::{DateTime, Utc};
use reqwest::{IntoUrl, Url};
use serde::Deserialize;
use serde::{de, de::Visitor, Deserializer};
use snafu::ResultExt;

/// This is a minimal pinboard API client with implementations only for endpoints needed by
/// pingrep.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: Url,
    client: reqwest::blocking::Client,
    token: String,
}

#[derive(Deserialize, Debug)]
struct Bookmark {
    /// The URL of the bookmark.
    pub href: String,
    /// The title of the bookmark.
    pub description: String,
    /// Extended description of the bookmark.
    pub extended: String,
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
    struct StringOrVector;

    impl<'de> Visitor<'de> for StringOrVector {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or space-separated string")
        }

        fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(s) = seq.next_element()? {
                vec.push(s);
            }
            Ok(vec)
        }

        fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(s.split(' ')
                .filter(|&tag| tag != "")
                .map(|tag| tag.to_owned())
                .collect())
        }
    }

    deserializer.deserialize_any(StringOrVector)
}

impl Client {
    /// Construct a new [`Client`] instance.
    pub fn new<U: IntoUrl>(base_url: U, token: &str) -> Result<Client> {
        Ok(Client {
            base_url: base_url.into_url().context(HttpSnafu {})?,
            client: reqwest::blocking::Client::builder()
                .user_agent("pingrep")
                .build()
                .context(HttpSnafu {})?,
            token: token.to_string(),
        })
    }

    /// Get the username associated with the client's token.
    pub fn user(&self) -> Result<String> {
        match self.token.split(':').next() {
            Some(user) => Ok(user.to_string()),
            None => MalformedTokenSnafu {
                reason: "Missing ':' in token",
            }
            .fail(),
        }
    }

    /// Get the time the user's bookmarks were last updated.
    pub fn last_update(&self) -> Result<DateTime<Utc>> {
        #[derive(Deserialize)]
        struct LastUpdateResponse {
            #[serde(rename = "update_time")]
            pub last_updated: DateTime<Utc>,
        }

        let url = self
            .base_url
            .join("posts/update")
            .expect("bad url in last_update");
        let params = [("auth_token", self.token.as_str()), ("format", "json")];
        let response = self
            .client
            .get(url)
            .query(&params)
            .send()
            .context(HttpSnafu {})?;
        response.error_for_status_ref().context(HttpSnafu {})?;
        Ok(response
            .json::<LastUpdateResponse>()
            .context(HttpSnafu {})?
            .last_updated)
    }

    /// Get all the user's bookmarks.
    pub fn bookmarks(&self) -> Result<Vec<super::Bookmark>> {
        let url = self
            .base_url
            .join("posts/all")
            .expect("bad url in last_update");
        let params = [("auth_token", self.token.as_str()), ("format", "json")];
        let response = self
            .client
            .get(url)
            .query(&params)
            .send()
            .context(HttpSnafu {})?;
        response.error_for_status_ref().context(HttpSnafu {})?;

        let bookmarks = response.json::<Vec<Bookmark>>().context(HttpSnafu {})?;
        Ok(bookmarks
            .into_iter()
            .map(|b| super::Bookmark {
                url: b.href,
                description: b.extended,
                title: b.description,
                tags: b.tags,
                time: b.time,
                shared: b.shared == "yes",
                toread: b.toread == "yes",
                meta: b.meta,
                hash: b.hash,
            })
            .collect())
    }
}
