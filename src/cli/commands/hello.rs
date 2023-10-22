use crate::cli::Context;
use crate::errors::*;
use clap::Parser;
use snafu::ResultExt;

/// Say hello
#[derive(Parser)]
pub struct Args {}

pub fn command(ctx: Context, _args: Args) -> Result<()> {
    ctx.api_token_entry
        .delete_password()
        .context(KeyRingSnafu {
            message: "Cannot set pinboard API token",
        })?;
    let password = ctx.api_token_entry.get_password().context(KeyRingSnafu {
        message: "Cannot read pinboard API token",
    })?;
    println!("API key: '{}'", password);
    Ok(())
}
