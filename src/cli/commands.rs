use crate::core::{NullScriptError, format_error};
use crate::cli::handler::CliHandler;
use clap::{Args, CommandFactory, Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "nsc",
    version = None,
    about = "NullScript transpiler - TypeScript with attitude",
    long_about = None,
    after_help = "Examples:
  nsc build src/                    # Transpile all .ns files in src/ to TypeScript
  nsc build src/ --js               # Transpile to JavaScript
  nsc run hello.ns                  # Run a NullScript file
  nsc check src/                    # Type-check NullScript files
  nsc keywords                      # Show all available keywords
  nsc system --info                 # Show system information
  nsc info src/ --detailed          # Show detailed file information

Learn more at: https://github.com/nullscript-lang/nullscript"
)]
pub struct Cli {
    #[arg(short = 'v', long = "version", help = "Print Version")]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Build(BuildArgs),
    Run(RunArgs),
    Check(CheckArgs),
    Keywords(KeywordsArgs),
    System(SystemArgs),
    Info(InfoArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    pub path: PathBuf,

    #[arg(short = 'o', long = "outDir", default_value = "dist")]
    pub out_dir: PathBuf,

    #[arg(long = "js")]
    pub js: bool,

    #[arg(long = "ts")]
    pub ts: bool,

    #[arg(long = "skip-type-check")]
    pub skip_type_check: bool,
}

#[derive(Args)]
pub struct RunArgs {
    pub file: PathBuf,

    #[arg(long = "skip-type-check")]
    pub skip_type_check: bool,
}

#[derive(Args)]
pub struct CheckArgs {
    pub path: PathBuf,
}

#[derive(Args)]
pub struct KeywordsArgs {
    #[arg(short = 'c', long = "category")]
    pub category: Option<String>,
}

#[derive(Args)]
pub struct SystemArgs {
    #[arg(short = 'i', long = "info", help = "Show system information")]
    pub info: bool,
}

#[derive(Args)]
pub struct InfoArgs {
    pub path: PathBuf,

    #[arg(short = 'd', long = "detailed", help = "Show detailed file information")]
    pub detailed: bool,
}

impl CliHandler {
    pub async fn handle_command(&self, command: Commands) -> Result<(), NullScriptError> {
        match command {
            Commands::Build(args) => self.handle_build(args.path, args.out_dir, args.js, args.skip_type_check).await,
            Commands::Run(args) => self.handle_run(args.file, args.skip_type_check).await,
            Commands::Check(args) => self.handle_check(args.path).await,
            Commands::Keywords(args) => self.handle_keywords(args.category),
            Commands::System(args) => self.handle_system(args),
            Commands::Info(args) => self.handle_info(args),
        }
    }
}

pub async fn run() -> Result<(), NullScriptError> {
    let cli = Cli::parse();

    if cli.version {
        println!("nsc v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let handler = CliHandler::new();

    if let Some(command) = cli.command {
        if let Err(e) = handler.handle_command(command).await {
            eprintln!("{}", format_error(&e).red());
            std::process::exit(1);
        }
    } else {
        let _ = Cli::command().print_help();
        std::process::exit(1);
    }

    Ok(())
}

impl CliHandler {
    pub fn check_node_availability() -> bool {
        Command::new("node").arg("--version").output().is_ok()
    }

    pub fn check_tsc_availability() -> bool {
        Command::new("tsc").arg("--version").output().is_ok()
    }

    pub fn show_system_info() {
        println!("{}", "🔧 System Information".cyan());
        println!("{}", "=".repeat(30).bright_black());
        println!("Node.js: {}", if Self::check_node_availability() { "✅ Available".green() } else { "❌ Not found".red() });
        println!("TypeScript: {}", if Self::check_tsc_availability() { "✅ Available".green() } else { "❌ Not found".red() });
        println!("NullScript: {} v{}", "✅ Available".green(), env!("CARGO_PKG_VERSION"));
    }

    pub fn handle_system(&self, args: SystemArgs) -> Result<(), NullScriptError> {
        if args.info {
            Self::show_system_info();
        } else {
            Self::show_system_info();
        }
        Ok(())
    }

    pub fn handle_info(&self, args: InfoArgs) -> Result<(), NullScriptError> {
        use crate::utils::files::FileUtils;

        if !args.path.exists() {
            eprintln!("{}", format!("❌ Path does not exist: {}", args.path.display()).red());
            std::process::exit(1);
        }

        println!("{}", "📁 File Information".cyan());
        println!("{}", "=".repeat(30).bright_black());
        println!("Path: {}", args.path.display());

        if let Some(ext) = FileUtils::get_extension(&args.path) {
            println!("Extension: {}", ext);
        }

        if let Some(stem) = FileUtils::get_stem(&args.path) {
            println!("Name: {}", stem);
        }

        if args.path.is_file() {
            let size = FileUtils::get_file_size(&args.path)?;
            println!("Size: {}", FileUtils::format_file_size(size));

            if args.detailed {
                let lines = FileUtils::count_lines(&args.path)?;
                println!("Lines: {}", lines);

                if let Ok(modified) = FileUtils::get_modified_time(&args.path) {
                    if let Ok(duration) = modified.elapsed() {
                        println!("Modified: {} ago", format_duration(duration));
                    }
                }
            }
        } else if args.path.is_dir() {
            let (total_files, nullscript_files) = self.get_file_stats(&args.path)?;
            println!("Total files: {}", total_files);
            println!("NullScript files: {}", nullscript_files);

            if args.detailed {
                println!();
                println!("{}", "📋 File Details:".cyan());
                println!("{}", "─".repeat(40).bright_black());

                let mut file_details = Vec::new();
                let mut total_size = 0u64;

                for entry in walkdir::WalkDir::new(&args.path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                {
                    let file_path = entry.path().to_path_buf();
                    let size = FileUtils::get_file_size(&file_path).unwrap_or(0);
                    total_size += size;

                    let relative_path = file_path.strip_prefix(&args.path)
                        .unwrap_or(&file_path)
                        .to_string_lossy()
                        .to_string();

                    let ext = FileUtils::get_extension(&file_path)
                        .unwrap_or_else(|| "no-ext".to_string());

                    let lines = FileUtils::count_lines(&file_path).unwrap_or(0);

                    let modified = FileUtils::get_modified_time(&file_path)
                        .ok()
                        .and_then(|m| m.elapsed().ok())
                        .map(|d| format_duration(d))
                        .unwrap_or_else(|| "unknown".to_string());

                    file_details.push((relative_path, ext, size, lines, modified));
                }

                file_details.sort_by(|a, b| a.0.cmp(&b.0));

                for (path, ext, size, lines, modified) in file_details {
                    let size_str = FileUtils::format_file_size(size);
                    let is_ns = ext == "ns";
                    let icon = if is_ns { "🎭" } else { "📄" };

                    println!("{} {:<30} {:<8} {:<8} {:<6} {}",
                        icon,
                        if path.len() > 28 { format!("{}...", &path[..25]) } else { path },
                        size_str,
                        format!("{}L", lines),
                        ext,
                        modified
                    );
                }

                println!("{}", "─".repeat(40).bright_black());
                println!("Total size: {}", FileUtils::format_file_size(total_size));
            }
        }

        Ok(())
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{} seconds", secs)
    } else if secs < 3600 {
        format!("{} minutes", secs / 60)
    } else if secs < 86400 {
        format!("{} hours", secs / 3600)
    } else {
        format!("{} days", secs / 86400)
    }
}
