use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle}; // 追加
use mix::commands::{
    self, CopyOutcome, ListEntry, SlashGenerationOutcome, SlashRequest, SlashTarget,
};
use mix::error::AppError;

#[derive(Parser)]
#[command(name = "mix")]
#[command(about = "Unified CLI for mix snippets and slash command generation")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Snippet name to copy when no subcommand is provided
    snippet: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available snippets
    List,
    /// Generate Codex/Claude/Gemini slash command assets
    Slash {
        #[arg(value_enum)]
        target: SlashArg,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SlashArg {
    All,
    Codex,
    Claude,
    Gemini,
}

fn main() {
    let cli = Cli::parse();

    let result = match (cli.command, cli.snippet) {
        (Some(Commands::List), _) => handle_list(),
        (Some(Commands::Slash { target }), _) => handle_slash(target),
        (None, Some(snippet)) => handle_copy(&snippet),
        (None, None) => {
            Cli::command().print_help().ok();
            println!();
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn handle_copy(name: &str) -> Result<(), AppError> {
    let CopyOutcome { key, relative_path, absolute_path } = commands::copy_snippet(name)?;
    println!("✅ Copied '{key}' from {relative_path} -> {}", absolute_path.display());
    Ok(())
}

fn handle_list() -> Result<(), AppError> {
    let entries = commands::list_snippets()?;
    if entries.is_empty() {
        println!("(no snippets found)");
        return Ok(());
    }

    println!("📚 Available snippets:\n");
    for ListEntry { key, relative_path, title, description } in entries {
        println!("- {key} ({relative_path})");
        if let Some(title) = title {
            println!("  • {title}");
        }
        if let Some(description) = description {
            println!("    {description}");
        }
    }
    Ok(())
}

fn handle_slash(target: SlashArg) -> Result<(), AppError> {
    let request = match target {
        SlashArg::All => SlashRequest::All,
        SlashArg::Codex => SlashRequest::Only(SlashTarget::Codex),
        SlashArg::Claude => SlashRequest::Only(SlashTarget::Claude),
        SlashArg::Gemini => SlashRequest::Only(SlashTarget::Gemini),
    };

    // プログレスバーの設定
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::with_template(
            // 黄緑色 (green) を使用したスタイル設定
            "{spinner:.green} [{elapsed_precise}] {bar:40.green/white} {pos:>3}/{len:3} {msg}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    pb.set_message("Generating assets...");

    // 実行（クロージャで進捗を受け取る）
    let artifacts = commands::generate_slash_commands(request, |current, total| {
        if current == 0 {
            pb.set_length(total as u64);
        }
        pb.set_position(current as u64);
    })?;

    // 完了処理
    pb.finish_and_clear();

    // 最後にサマリーを表示
    println!("✨ Generated {} artifact(s):", artifacts.len());
    for SlashGenerationOutcome { target, path } in artifacts {
        // pureのような少しリッチな表示にするならここも色付け可能ですが、
        // まずは出力順序の修正を優先しています。
        println!("- {:>6} -> {}", target.label(), path.display());
    }

    Ok(())
}
