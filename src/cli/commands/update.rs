use crate::cli::Context;
use crate::errors::*;
use crate::pinboard::BookmarkCollection;
use chrono::{DateTime, Utc};
use clap::Parser;
use snafu::ResultExt;
use std::fs::File;

/// Update the local cache of bookmarks from pinboard.
#[derive(Parser)]
pub struct Args {}

pub fn command(ctx: Context, _args: Args) -> WhateverResult<()> {
    let client = ctx
        .get_pinboard_client()
        .whatever_context("Cannot initialize pinboard client")?;
    let local_timestamp = get_last_update_timestamp(&ctx)
        .whatever_context("Unable to get last update timestamp from bookmarks")?;
    let remote_timestamp = client
        .last_update()
        .whatever_context("Unable to get last update timestamp from pinboard")?;

    if let Some(timestamp) = local_timestamp {
        if timestamp >= remote_timestamp {
            if ctx.verbose {
                println!("Bookmarks are up to date (last changed {})", timestamp);
            }
            return Ok(());
        }
    };

    let collection = BookmarkCollection::from_api(&client)
        .whatever_context("Unable to get bookmarks from pinboard")?;

    save_collection(&ctx, &collection).whatever_context("Unable to save bookmarks")?;

    println!("Bookmarks updated (last changed {})", remote_timestamp);
    Ok(())
}

/// Save the given collection to disk.
fn save_collection(ctx: &Context, collection: &BookmarkCollection) -> Result<()> {
    let tmpfile = tempfile::Builder::new().tempfile_in(ctx.appdir.clone())?;

    collection.write(&tmpfile).context(PinboardClientSnafu {
        message: "cannot write bookmarks to temporary file".to_owned(),
    })?;

    let p = tmpfile.path().to_owned();
    tmpfile
        .persist(ctx.bookmark_file.clone())
        .context(PersistSnafu {
            dest: ctx.bookmark_file.clone(),
            path: p,
        })?;

    Ok(())
}

/// Get the timestamp of the last update of the bookmarks.
/// Returns `None` if the bookmark file does not exist.
fn get_last_update_timestamp(ctx: &Context) -> Result<Option<DateTime<Utc>>> {
    let f = match File::open(&ctx.bookmark_file) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Ok(None);
            }
            return Err(e).context(ReadSnafu {
                path: Some(ctx.bookmark_file.clone()),
            });
        }
    };

    let collection = BookmarkCollection::read(f).context(PinboardClientSnafu {
        message: "cannot read bookmarks from file".to_owned(),
    })?;
    Ok(Some(collection.last_updated))
}
