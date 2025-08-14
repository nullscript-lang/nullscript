use crate::errors::types::NullScriptError;
use crate::errors::formatting::ErrorFormatter;
use crate::cli::handler::CliHandler;
use clap::{Args, CommandFactory, Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "nsc",
    version = None,
    about = "NullScript transpiler - JavaScript with attitude",
    long_about = None,
    after_help = "Command Groups:

📦 BUILD & RUN:
  nsc build src/                    # Transpile all .ns files in src/ to JavaScript
  nsc run hello.ns                  # Run a NullScript file directly
  nsc convert app.js                # Convert JavaScript to NullScript

🔧 PROJECT MANAGEMENT:
  nsc init my-project               # Initialize a new NullScript project
  nsc config --generate             # Generate default configuration file
  nsc config --show                 # Show current configuration

💻 DEVELOPMENT:
  nsc dev src/ --watch              # Watch files and rebuild on changes
  nsc complete src/app.ns --line 10 --column 5  # Get code completion
  nsc debug app.ns --breakpoint 15  # Debug with breakpoint

📊 ANALYSIS & INFO:
  nsc analyze src/                  # Analyze project performance
  nsc analyze-clean                 # Remove all analysis reports
  nsc analytics .                   # Show project analytics
  nsc info src/ --detailed          # Show detailed file information

🎛️ UTILITIES:
  nsc keywords                      # Show all available keywords
  nsc system --info                 # Show system information

Learn more at: https://nullscript.js.org"
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
    Keywords(KeywordsArgs),
    System(SystemArgs),
    Info(InfoArgs),
    Config(ConfigArgs),
    Init(InitArgs),
    Complete(CompleteArgs),
    Dev(DevArgs),
    Analyze(AnalyzeArgs),
    AnalyzeClean(AnalyzeCleanArgs),
    Debug(DebugArgs),
    Convert(ConvertArgs),
    Analytics(AnalyticsArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    pub path: PathBuf,

    #[arg(short = 'o', long = "outDir", default_value = "dist")]
    pub out_dir: PathBuf,
}

#[derive(Args)]
pub struct RunArgs {
    pub file: PathBuf,
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

#[derive(Args)]
pub struct ConfigArgs {
    #[arg(short = 's', long = "show", help = "Show current configuration")]
    pub show: bool,

    #[arg(short = 'g', long = "generate", help = "Generate default configuration file")]
    pub generate: bool,

    #[arg(short = 'v', long = "validate", help = "Validate configuration file")]
    pub validate: bool,
}

#[derive(Args)]
pub struct InitArgs {
    #[arg(help = "Project name")]
    pub name: Option<String>,

    #[arg(short = 't', long = "template", help = "Project template")]
    pub template: Option<String>,

    #[arg(long = "force", help = "Force initialization in non-empty directory")]
    pub force: bool,
}

#[derive(Args)]
pub struct CompleteArgs {
    #[arg(help = "File path")]
    pub file: PathBuf,

    #[arg(short = 'l', long = "line", help = "Line number (1-based)")]
    pub line: u32,

    #[arg(short = 'c', long = "column", help = "Column number (1-based)")]
    pub column: u32,

    #[arg(long = "format", default_value = "json", help = "Output format (json, text)")]
    pub format: String,
}

#[derive(Args)]
pub struct DevArgs {
    #[arg(help = "Directory to watch", default_value = "src")]
    pub path: PathBuf,

    #[arg(short = 'w', long = "watch", help = "Watch files and run on changes")]
    pub watch: bool,

    #[arg(long = "run-on-save", help = "Execute the file when it changes")]
    pub run_on_save: bool,
}

#[derive(Args)]
pub struct AnalyzeArgs {
    #[arg(help = "Input directory to analyze", default_value = "src")]
    pub path: PathBuf,

    #[arg(short = 'o', long = "output", help = "Output directory for reports", default_value = "reports")]
    pub output: PathBuf,

    #[arg(long = "format", help = "Report format", default_value = "html")]
    pub format: String,

    #[arg(long = "bundle-size-limit", help = "Bundle size limit in bytes")]
    pub bundle_size_limit: Option<u64>,

    #[arg(long = "build-time-budget", help = "Build time budget in milliseconds")]
    pub build_time_budget: Option<u32>,
}

#[derive(Args)]
pub struct DebugArgs {
    #[arg(help = "NullScript file to debug")]
    pub file: PathBuf,

    #[arg(long = "breakpoint", help = "Set initial breakpoint at line")]
    pub breakpoint: Option<u32>,

    #[arg(long = "profile", help = "Enable performance profiling")]
    pub profile: bool,
}

#[derive(Args)]
pub struct ConvertArgs {
    #[arg(help = "JavaScript file to convert to NullScript")]
    pub file: PathBuf,

    #[arg(short = 'o', long = "output", help = "Output file path")]
    pub output: Option<PathBuf>,

    #[arg(long = "format", help = "Format the output code")]
    pub format: bool,

    #[arg(long = "report", help = "Show conversion report")]
    pub report: bool,
}

#[derive(Args)]
pub struct AnalyticsArgs {
    #[arg(help = "Project directory to analyze", default_value = ".")]
    pub path: PathBuf,

    #[arg(long = "days", help = "Number of days to analyze", default_value = "30")]
    pub days: u32,

    #[arg(long = "format", help = "Output format", default_value = "text")]
    pub format: String,
}

#[derive(Args)]
pub struct AnalyzeCleanArgs {
    #[arg(long = "reports-dir", help = "Reports directory to clean", default_value = "reports")]
    pub reports_dir: PathBuf,

    #[arg(short = 'f', long = "force", help = "Force removal without confirmation")]
    pub force: bool,
}

impl CliHandler {
    pub async fn handle_command(&self, command: Commands) -> Result<(), NullScriptError> {
        match command {
            Commands::Build(args) => self.handle_build(args.path, args.out_dir).await,
            Commands::Run(args) => self.handle_run(args.file).await,
            Commands::Keywords(args) => self.handle_keywords(args.category),
            Commands::System(args) => self.handle_system(args),
            Commands::Info(args) => self.handle_info(args),
            Commands::Config(args) => self.handle_config(args),
            Commands::Init(args) => self.handle_init(args),
            Commands::Complete(args) => self.handle_complete(args),
            Commands::Dev(args) => self.handle_dev(args).await,
            Commands::Analyze(args) => self.handle_analyze(args).await,
            Commands::AnalyzeClean(args) => self.handle_analyze_clean(args).await,
            Commands::Debug(args) => self.handle_debug(args).await,
            Commands::Convert(args) => self.handle_convert(args).await,
            Commands::Analytics(args) => self.handle_analytics(args).await,
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
            eprintln!("{}", ErrorFormatter::format_error(&e));
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

    pub fn show_system_info() {
        println!("{}", "🔧 System Information".cyan());
        println!("{}", "=".repeat(30).bright_black());
        println!("Node.js: {}", if Self::check_node_availability() { "✅ Available".green() } else { "❌ Not found".red() });
        println!("NullScript: {} v{}", "✅ Available".green(), env!("CARGO_PKG_VERSION"));
    }

    pub fn handle_system(&self, _args: SystemArgs) -> Result<(), NullScriptError> {
        Self::show_system_info();
        Ok(())
    }

    pub fn handle_info(&self, args: InfoArgs) -> Result<(), NullScriptError> {
        use crate::common::files::FileUtils;

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
                        .map(format_duration)
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
