use super::{Bookmark, HttpSnafu, Result};
use reqwest::{IntoUrl, Url};
use serde::Deserialize;
use snafu::ResultExt;

/// This is a minimal pinboard API client with implementations only for endpoints needed by
/// pingrep.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: Url,
    client: reqwest::blocking::Client,
    token: String,
}

/// The response from the [`Client::last_update`] method, as returned by the pinboard API.
#[derive(Clone, Copy, Deserialize, Debug)]
pub struct LastUpdateResponse {
    #[serde(rename = "update_time")]
    pub last_updated: chrono::DateTime<chrono::Utc>,
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

    /// Get the time the user's bookmarks were last updated.
    pub fn last_update(&self) -> Result<LastUpdateResponse> {
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
        response.json::<LastUpdateResponse>().context(HttpSnafu {})
    }

    /// Get all the user's bookmarks.
    pub fn bookmarks(&self) -> Result<Vec<Bookmark>> {
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
        response.json::<Vec<Bookmark>>().context(HttpSnafu {})
    }
}
