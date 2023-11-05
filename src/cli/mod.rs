mod commands;
mod filters;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use super::errors::*;
use super::pinboard;
use super::pinboard::client::Client;
use crate::subcommands;

use clap::{Parser, Subcommand};
use commands::*;
use directories::ProjectDirs;
use snafu::OptionExt;
use snafu::ResultExt;

/// The global command context
#[derive(Debug)]
pub struct Context {
    /// Enable debug statements
    pub verbose: bool,
    /// The application directory
    pub appdir: PathBuf,
    /// The bookmark collection file under appdir
    pub bookmark_file: PathBuf,
    /// The pinboard API token keyring entry
    pub api_token_entry: keyring::Entry,
}

/// Search pinboard bookmarks from your CLI
#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,

    /// Enable more verbose output
    #[clap(global = true, short, long)]
    verbose: bool,
}

subcommands!(fzf, hello, login, open, show, update);

pub fn main() -> WhateverResult<()> {
    let args = Args::parse();
    let appdir = initialize_appdir().whatever_context("cannot initialize appdir")?;

    let ctx = Context {
        appdir: appdir.clone(),
        bookmark_file: appdir.join("bookmarks.json"),
        api_token_entry: initialize_keyring("pinboard-api-token")
            .whatever_context("cannot initialize keyring")?,
        verbose: args.verbose,
    };
    Commands::run(ctx, args)
}

/// Initialize the application directory, ensuring it exists.
fn initialize_appdir() -> Result<PathBuf> {
    let projectdir =
        ProjectDirs::from("com.github", "zoni", "pingrep").context(AppDirLookupSnafu)?;
    let appdir = projectdir.data_dir();

    std::fs::create_dir_all(appdir).context(IOSnafu {
        path: appdir.to_path_buf(),
    })?;

    Ok(appdir.to_path_buf())
}

fn initialize_keyring(key: &str) -> Result<keyring::Entry> {
    keyring::Entry::new("pingrep", key).context(KeyRingSnafu {
        message: "Cannot initialize keyring",
    })
}

impl Context {
    fn get_pinboard_client(&self) -> Result<Client> {
        let token = self.api_token_entry.get_password().context(KeyRingSnafu {
            message: "Cannot get pinboard API token from keyring",
        })?;

        let client =
            Client::new(pinboard::PINBOARD_API_URL, &token).context(PinboardClientSnafu {
                message: "Cannot initialize pinboard client",
            })?;
        Ok(client)
    }

    fn read_bookmarks(&self) -> Result<pinboard::BookmarkCollection> {
        let file = File::open(&self.bookmark_file).context(ReadSnafu {
            path: self.bookmark_file.clone(),
        })?;
        let reader = BufReader::new(file);
        let collection =
            pinboard::BookmarkCollection::read(reader).context(PinboardClientSnafu {
                message: "Cannot read bookmarks from file",
            })?;
        Ok(collection)
    }
}
