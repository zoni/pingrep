use crate::cli::Context;
use crate::errors::*;
use clap::Parser;
use snafu::ResultExt;

/// Say hello
#[derive(Parser)]
pub struct Args {}

pub fn command(ctx: Context, _args: Args) -> WhateverResult<()> {
    let client = ctx
        .get_pinboard_client()
        .whatever_context("Cannot initialize pinboard client")?;
    let bookmarks = client
        .bookmarks()
        .whatever_context("Unable to get bookmarks from pinboard")?;

    println!("{:?}", bookmarks);

    //println!("Hello world");
    Ok(())
}
