use std::thread;

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
    collection: BookmarkCollection,
}

pub fn command(ctx: Context, _args: Args) -> WhateverResult<()> {
    let collection = ctx
        .read_bookmarks()
        .whatever_context("Unable to read bookmarks")?;

    let template = FzfTemplate {
        separator: FZF_FIELD_SEPARATOR,
        collection,
    };
    let (pipereader, mut pipewriter) = os_pipe::pipe().whatever_context("Unable to create pipe")?;
    let template_writer = thread::spawn(move || template.write_into(&mut pipewriter));

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
        "ctrl-y:execute(echo {2} | cbcopy)",
        "--bind",
        format!("enter:become({} open {{2}})", exe),
        "--bind",
        format!("ctrl-o:execute({} open {{2}})", exe),
        "--bind",
        format!("ctrl-e:execute({} open {{5}})", exe),
    );
    let handle = fzf
        .stdin_file(pipereader)
        .unchecked()
        .start()
        .whatever_context("unable to start fzf")?;

    if let Ok(result) = handle.wait() {
        match result.status.code() {
            Some(0) => (),
            // fzf returns 130 when selection is cancelled.
            Some(130) => (),
            Some(code) => whatever!("fzf exited with returncode {}", code),
            _ => whatever!("fzf terminated by signal"),
        }
    }
    template_writer
        .join()
        .expect("Unable to join template writer thread")
        .whatever_context("Failed to write bookmarks to fzf")?;

    Ok(())
}
