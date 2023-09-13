use std::fs::File;
use std::io::Write;
use std::iter::once;
use std::path::PathBuf;
use std::process::Command;

use clap::{Args, Parser, Subcommand};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct NewArgs {
    url: String,
}
#[derive(Subcommand)]
enum Commands {
    New(NewArgs),
}

#[derive(Debug)]
enum NewError {
    CannotFetchPage,
    BrokenPage,
    CannotFindPattern,
    CannotCreateFile,
    CannotWriteToFile,
}
fn new(args: NewArgs) -> Result<(), NewError> {
    let re = Regex::new(r#"<span>(\d+) kyu</span></div></div><h4 class="ml-2 mb-3">(.+?)</h4>"#)
        .unwrap();
    let response = reqwest::blocking::get(&args.url).map_err(|_| NewError::CannotFetchPage)?;
    let text = response.text().map_err(|_| NewError::BrokenPage)?;
    let (_, [kyu, name]) = re
        .captures_iter(&text)
        .map(|c| c.extract())
        .next()
        .ok_or(NewError::CannotFindPattern)?;
    let project_name = name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ')
        .map(|c| match c {
            ' ' => '_',
            _ => c,
        })
        .chain(once('-'))
        .chain(kyu.chars())
        .collect::<String>();
    Command::new("cargo")
        .arg("new")
        .arg(&project_name)
        .output()
        .expect("Failed to execute command");
    let mut url_file = PathBuf::from(&project_name);
    url_file.push("url.txt");
    let mut file = File::create(url_file).map_err(|_| NewError::CannotCreateFile)?;
    file.write_all(&args.url.as_bytes())
        .map_err(|_| NewError::CannotWriteToFile)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New(args) => match new(args) {
            Ok(_) => {}
            Err(e) => eprintln!("{e:?}"),
        },
    }
}
