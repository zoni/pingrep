mod commands;

use snafu::OptionExt;
use snafu::ResultExt;
use std::path::PathBuf;

use super::errors::*;
use super::pinboard;
use super::pinboard::client::Client;
use crate::subcommands;
use clap::{Parser, Subcommand};
use commands::*;
use directories::ProjectDirs;

/// The global command context
pub struct Context {
    /// Enable debug statements
    pub verbose: bool,
    /// The application directory
    pub appdir: PathBuf,
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

subcommands!(login, update, hello);

pub fn main() -> WhateverResult<()> {
    let args = Args::parse();
    let ctx = Context {
        appdir: initialize_appdir().whatever_context("cannot initialize appdir")?,
        api_token_entry: initialize_keyring("pinboard-api-token")
            .whatever_context("cannot initialize keyring")?,
        verbose: args.verbose,
    };
    Commands::run(ctx, args)
}

fn initialize_appdir() -> Result<PathBuf> {
    let projectdir =
        ProjectDirs::from("com.github", "zoni", "pingrep-rs").context(AppDirLookupSnafu)?;
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
    fn get_pinboard_client(self) -> Result<Client> {
        let token = self.api_token_entry.get_password().context(KeyRingSnafu {
            message: "Cannot get pinboard API token from keyring",
        })?;

        let client =
            Client::new(pinboard::PINBOARD_API_URL, &token).context(PinboardClientSnafu {
                message: "Cannot initialize pinboard client",
            })?;
        Ok(client)
    }
}
