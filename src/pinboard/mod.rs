pub mod client;

use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

pub const PINBOARD_API_URL: &str = "https://api.pinboard.in/v1/";

#[non_exhaustive]
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("HTTP request error: {source}"))]
    HttpError { source: reqwest::Error },
}
