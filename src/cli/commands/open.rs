use crate::cli::Context;
use crate::errors::*;
use clap::Parser;
use snafu::whatever;
use snafu::ResultExt;

/// Open a given bookmark in a web browser
#[derive(Parser)]
pub struct Args {
    /// The bookmark to open
    url: String,
}

pub fn command(ctx: Context, args: Args) -> WhateverResult<()> {
    let collection = ctx
        .read_bookmarks()
        .whatever_context("Unable to read bookmarks")?;

    match collection.find_by_url(&args.url) {
        Some(bookmark) => {
            open::that_detached(bookmark.url.clone()).whatever_context("Unable to open URL")
        }
        None => whatever!("Bookmark not found"),
    }
}
