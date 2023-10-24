use crate::cli::Context;
use crate::errors::*;
use clap::Parser;
use snafu::ResultExt;

/// Set the pinboard API token (saved to system keyring)
#[derive(Parser)]
pub struct Args {}

pub fn command(ctx: Context, _args: Args) -> WhateverResult<()> {
    let client = ctx
        .get_pinboard_client()
        .whatever_context("Cannot initialize pinboard client")?;
    let resp = client
        .last_update()
        .whatever_context("Unable to get last update time from pinboard")?;
    println!("Last update: {}", resp.last_updated);
    Ok(())
}
