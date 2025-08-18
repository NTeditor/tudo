mod shell;
use std::{env, process};

use clap::Parser;
use shell::Shell;

const USAGE: &'static str = "tudo [FLAGS] [COMMAND]\n       tudo [FLAGS] --login";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, override_usage = USAGE)]
struct Args {
    #[arg(required_unless_present = "login", conflicts_with = "login")]
    command: Option<Vec<String>>,

    #[arg(short, long)]
    login: bool,

    #[arg(short, long)]
    shell: Option<String>,
}

fn main() {
    let args = Args::parse();
    let shell = match args.shell {
        Some(value) => value,
        None => env::var("SHELL").unwrap_or("/system/bin/sh".to_string()),
    };

    let mut command = Shell::new(shell, args.command).new_shell();
    let status = match command.status() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Failed start shell {}", e);
            process::exit(1);
        }
    };

    process::exit(status.code().unwrap_or(1))
}
