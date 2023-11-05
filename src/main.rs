use pingrep::cli;
use pingrep::errors::WhateverResult;

#[snafu::report]
fn main() -> WhateverResult<()> {
    cli::main()?;
    Ok(())
}
