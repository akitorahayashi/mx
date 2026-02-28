mod add;
mod cat;
mod checkout;
mod clean;
mod copy;
mod list;
mod remove;
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
    #[command(about = "List available snippets", visible_alias = "ls")]
    List,
    #[command(about = "Create context files", visible_alias = "t")]
    Touch {
        key: String,
        #[arg(short = 'f', long = "force")]
        force: bool,
    },
    #[command(about = "Display context file contents", visible_alias = "ct")]
    Cat { key: String },
    #[command(about = "Clean context files or directory", visible_alias = "cl")]
    Clean { key: Option<String> },
    #[command(about = "Copy a snippet to the clipboard", visible_alias = "c")]
    Copy { snippet: String },
    #[command(about = "Check out snippet(s) as symlinks in .mx/commands/", visible_alias = "co")]
    Checkout {
        path: Option<String>,
        #[arg(short = 'a', long)]
        all: bool,
    },
    #[command(about = "Add a snippet from clipboard", visible_aliases = ["a", "ad"])]
    Add {
        path: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(short = 'f', long)]
        force: bool,
    },
    #[command(about = "Remove a snippet", visible_alias = "rm")]
    Remove { snippet: String },
}

pub fn run() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::List) => list::run(),
        Some(Commands::Touch { key, force }) => touch::run(&key, force),
        Some(Commands::Cat { key }) => cat::run(&key),
        Some(Commands::Clean { key }) => clean::run(key),
        Some(Commands::Copy { snippet }) => copy::run(&snippet),
        Some(Commands::Checkout { path, all }) => checkout::run(path.as_deref(), all),
        Some(Commands::Add { path, title, description, force }) => {
            add::run(&path, title.as_deref(), description.as_deref(), force)
        }
        Some(Commands::Remove { snippet }) => remove::run(&snippet),
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
