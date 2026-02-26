mod cat;
mod clean;
mod copy;
mod list;
mod touch;

use crate::domain::error::AppError;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mx")]
#[command(version)]
#[command(about = "Unified CLI for mx snippets")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "ls")]
    List,
    #[command(visible_alias = "t")]
    Touch {
        key: String,
        #[arg(short = 'f', long = "force")]
        force: bool,
    },
    #[command(visible_alias = "ct")]
    Cat { key: String },
    #[command(visible_alias = "cl")]
    Clean { key: Option<String> },
    #[command(visible_alias = "c")]
    Copy { snippet: String },
}

pub fn run() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::List) => list::run(),
        Some(Commands::Touch { key, force }) => touch::run(&key, force),
        Some(Commands::Cat { key }) => cat::run(&key),
        Some(Commands::Clean { key }) => clean::run(key),
        Some(Commands::Copy { snippet }) => copy::run(&snippet),
        None => {
            Cli::command().print_help().ok();
            println!();
            Ok(())
        }
    };

    if let Err(err) = result {
        report_error_and_exit(err);
    }
}

fn report_error_and_exit(err: AppError) {
    eprintln!("Error: {err}");
    std::process::exit(1);
}
