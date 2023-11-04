use crate::cli::filters;
use crate::cli::Context;
use crate::errors::*;
use crate::pinboard::Bookmark;
use askama::Template;
use clap::Parser;
use snafu::whatever;
use snafu::ResultExt;

/// Show a given bookmark
#[derive(Parser)]
pub struct Args {
    /// The bookmark to show
    url: String,
}

#[derive(Template)]
#[template(path = "show.txt")]
struct ShowTemplate<'a> {
    user: &'a str,
    bookmark: &'a Bookmark,
}

pub fn command(ctx: Context, args: Args) -> WhateverResult<()> {
    let collection = ctx
        .read_bookmarks()
        .whatever_context("Unable to read bookmarks")?;

    match collection.find_by_url(&args.url) {
        Some(bookmark) => {
            let template = ShowTemplate {
                user: &collection.user,
                bookmark: &bookmark,
            };
            println!(
                "{}",
                template
                    .render()
                    .whatever_context("Unable to render template")?
            );
            Ok(())
        }
        None => whatever!("Bookmark not found"),
    }
}
