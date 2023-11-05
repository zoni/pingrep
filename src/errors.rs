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

    #[snafu(display("IO error{}", path.clone().map_or("".to_owned(), |p| format!(" on {:?}", p))))]
    #[allow(clippy::upper_case_acronyms)]
    IOError {
        path: Option<PathBuf>,
        source: std::io::Error,
    },

    #[snafu(display("Read error{}", path.clone().map_or("".to_owned(), |p| format!(" on {:?}", p))))]
    ReadError {
        path: Option<PathBuf>,
        source: std::io::Error,
    },

    #[snafu(display("Write error{}", path.clone().map_or("".to_owned(), |p| format!(" on {:?}", p))))]
    WriteError {
        path: Option<PathBuf>,
        source: std::io::Error,
    },

    #[snafu(display("Unable to persist temporary file {:?}{}", path.display(), dest.clone().map_or("".to_owned(), |p| format!(" to {:?}", p))))]
    PersistError {
        path: PathBuf,
        dest: Option<PathBuf>,
        source: tempfile::PersistError,
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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError {
            path: None,
            source: e,
        }
    }
}

impl From<tempfile::PersistError> for Error {
    fn from(e: tempfile::PersistError) -> Self {
        Error::PersistError {
            path: e.file.path().to_path_buf(),
            dest: None,
            source: e,
        }
    }
}
