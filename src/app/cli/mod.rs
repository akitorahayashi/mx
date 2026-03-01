use crate::app::commands;
use crate::domain::error::AppError;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mx")]
#[command(version)]
#[command(about = "Unified CLI for mx snippets")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "List available snippets", visible_alias = "ls")]
    List(commands::list::Cli),
    #[command(about = "Create context files", visible_alias = "t")]
    Touch(commands::touch::Cli),
    #[command(about = "Display context file contents", visible_alias = "ct")]
    Cat(commands::cat::Cli),
    #[command(about = "Clean context files or directory", visible_alias = "cl")]
    Clean(commands::clean::Cli),
    #[command(about = "Copy a snippet to the clipboard", visible_alias = "c")]
    Copy(commands::copy::Cli),
    #[command(about = "Check out snippet(s) as symlinks in .mx/commands/", visible_alias = "co")]
    Checkout(commands::checkout::Cli),
    #[command(about = "Add a snippet from clipboard", visible_aliases = ["a", "ad"])]
    Add(commands::add::Cli),
    #[command(about = "Remove a snippet", visible_alias = "rm")]
    Remove(commands::remove::Cli),
    #[command(about = "Create a new command template in .mx/commands/", visible_alias = "cc")]
    CreateCommand(commands::create_command::Cli),
}

pub fn run() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::List(args)) => commands::list::run(args),
        Some(Commands::Touch(args)) => commands::touch::run(args),
        Some(Commands::Cat(args)) => commands::cat::run(args),
        Some(Commands::Clean(args)) => commands::clean::run(args),
        Some(Commands::Copy(args)) => commands::copy::run(args),
        Some(Commands::Checkout(args)) => commands::checkout::run(args),
        Some(Commands::Add(args)) => commands::add::run(args),
        Some(Commands::Remove(args)) => commands::remove::run(args),
        Some(Commands::CreateCommand(args)) => commands::create_command::run(args),
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
