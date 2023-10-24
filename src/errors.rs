use super::pinboard;
use snafu::{Snafu, Whatever};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;
pub type WhateverResult<T> = std::result::Result<T, Whatever>;

#[non_exhaustive]
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[snafu(display("{}", message))]
    GeneralError {
        message: String,
        //#[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        //source: Option<Box<dyn std::error::Error>>,
    },

    #[snafu(display("Unable to determine application directory"))]
    AppDirLookupError {},

    #[snafu(display("IO error on '{}'", path.display()))]
    #[allow(clippy::upper_case_acronyms)]
    IOError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Read error on '{}'", path.display()))]
    ReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Write error on '{}'", path.display()))]
    WriteError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to execute {} with args {:?}", cmd, args))]
    ExecError {
        cmd: String,
        args: Vec<String>,
        source: std::io::Error,
    },

    #[snafu(display("Failed to read environment variable {}", key))]
    EnvVarError {
        key: String,
        source: std::env::VarError,
    },

    #[snafu(display("{message}"))]
    KeyRingError {
        message: String,
        source: keyring::error::Error,
    },

    #[snafu(display("{message}"))]
    PinboardClientError {
        message: String,
        source: pinboard::Error,
    },

    #[snafu(display("Not implemented"))]
    NotImplemented,
}

impl From<GeneralSnafu<std::string::String>> for Error {
    fn from(e: GeneralSnafu<std::string::String>) -> Self {
        Error::GeneralError { message: e.message }
    }
}
