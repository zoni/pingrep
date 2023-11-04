use crate::cli::Context;
use crate::errors::*;
use clap::Parser;
use snafu::ResultExt;

/// Open a given bookmark in a web browser
#[derive(Parser)]
pub struct Args {
    /// The bookmark to open
    url: String,
}

pub fn command(_ctx: Context, args: Args) -> WhateverResult<()> {
    open::that_detached(args.url).whatever_context("Unable to open URL")
}
