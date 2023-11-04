use crate::cli::filters;
use crate::cli::Context;
use crate::errors::*;
use crate::pinboard::BookmarkCollection;
use askama::Template;
use clap::Parser;
use duct::cmd;
use snafu::whatever;
use snafu::{OptionExt, ResultExt};

const FZF_FIELD_SEPARATOR: &str = "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t";

/// Show a given bookmark
#[derive(Parser)]
pub struct Args {}

#[derive(Template)]
#[template(path = "fzf.txt")]
struct FzfTemplate<'a> {
    separator: &'a str,
    collection: &'a BookmarkCollection,
}

pub fn command(ctx: Context, _args: Args) -> WhateverResult<()> {
    let collection = ctx
        .read_bookmarks()
        .whatever_context("Unable to read bookmarks")?;

    let template = FzfTemplate {
        separator: FZF_FIELD_SEPARATOR,
        collection: &collection,
    };
    let input = template
        .render()
        .whatever_context("Unable to render input for fzf")?;

    let exe = std::env::current_exe().whatever_context("unable to determine exe")?;
    let exe = exe
        .to_str()
        .whatever_context("unable to convert executable path to string")?;
    let fzf = cmd!(
        "fzf",
        "--delimiter",
        FZF_FIELD_SEPARATOR,
        "--ellipsis",
        "",
        "--no-hscroll",
        "--preview",
        format!("{} show {{2}}", exe),
        "--bind",
        "ctrl-y:execute-silent(echo {2} | cbcopy)",
        "--bind",
        format!("enter:become({} browse {{2}})", exe),
        "--bind",
        format!("ctrl-o:execute-silent({} browse {{2}})", exe),
        "--bind",
        format!("ctrl-e:execute-silent({} browse {{5}})", exe),
    );
    let handle = fzf
        .stdin_bytes(input.as_bytes())
        .unchecked()
        .start()
        .whatever_context("unable to start fzf")?;

    if let Ok(result) = handle.wait() {
        match result.status.code() {
            Some(130) => {
                // fzf returns 130 when selection is cancelled.
                return Ok(());
            }
            Some(code) => whatever!("fzf exited with returncode {}", code),
            _ => whatever!("fzf terminated by signal"),
        }
    }

    Ok(())
}
