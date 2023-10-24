use crate::cli::Context;
use crate::errors::*;
use clap::Parser;

/// Say hello
#[derive(Parser)]
pub struct Args {}

pub fn command(_ctx: Context, _args: Args) -> WhateverResult<()> {
    println!("Hello world");
    Ok(())
}
