use crate::cli::Context;
use crate::errors::*;
use crate::pinboard;
use clap::Parser;
use rpassword::prompt_password;
use snafu::ResultExt;

/// Set the pinboard API token (saved to system keyring)
#[derive(Parser)]
pub struct Args {}

pub fn command(ctx: Context, _args: Args) -> Result<()> {
    let password = prompt_password("Enter API key from https://pinboard.in/settings/password: ")
        .context(PasswordPromptSnafu)?;

    let client = pinboard::Client::new(pinboard::PINBOARD_API_URL, &password).context(
        PinboardClientSnafu {
            message: "Cannot initialize pinboard client",
        },
    )?;

    ctx.api_token_entry
        .set_password(&password)
        .context(KeyRingSnafu {
            message: "Failed to save token to keyring",
        })?;
    let _resp = client.last_update().context(PinboardLoginSnafu)?;

    if ctx.verbose {
        println!("Login successful");
    }
    Ok(())
}
