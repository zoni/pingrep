use crate::cli::Context;
use crate::errors::WhateverResult;
use crate::pinboard;
use crate::pinboard::client::Client;
use clap::Parser;
use rpassword::prompt_password;
use snafu::prelude::*;

/// Set the pinboard API token (saved to system keyring)
#[derive(Parser)]
pub struct Args {}

pub fn command(ctx: Context, _args: Args) -> WhateverResult<()> {
    let password = prompt_password("Enter API key from https://pinboard.in/settings/password: ")
        .whatever_context("Failed to read password")?;

    let client = Client::new(pinboard::PINBOARD_API_URL, &password)
        .whatever_context("Cannot initialize pinboard client")?;

    ctx.api_token_entry
        .set_password(&password)
        .whatever_context("Failed to save token to keyring")?;
    let _resp = client
        .last_update()
        .whatever_context("pinboard login failed")?;

    if ctx.verbose {
        println!("Login successful");
    }
    Ok(())
}
