#![allow(dead_code)]
extern crate clap;

use crate::args::args::Cli;
use crate::clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::fs;

#[path = "src/args/mod.rs"]
mod args;

fn main() {
    let mut command = Cli::command();
    fs::create_dir_all("completions").unwrap();
    generate_to(Shell::Zsh, &mut command, "speedbag", "completions").unwrap();
    generate_to(Shell::Bash, &mut command, "speedbag", "completions").unwrap();
}
