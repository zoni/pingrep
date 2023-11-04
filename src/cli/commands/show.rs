use crate::cli::Context;
use crate::errors::*;
use clap::Parser;
use snafu::whatever;
use snafu::ResultExt;

/// Show a given bookmark
#[derive(Parser)]
pub struct Args {
    /// The bookmark to show
    url: String,
}

pub fn command(ctx: Context, args: Args) -> WhateverResult<()> {
    let collection = ctx
        .read_bookmarks()
        .whatever_context("Unable to read bookmarks")?;

    match collection.find_by_url(&args.url) {
        Some(bookmark) => {
            println!("{:#?}", bookmark);
            Ok(())
        }
        None => whatever!("Bookmark not found"),
    }
}
