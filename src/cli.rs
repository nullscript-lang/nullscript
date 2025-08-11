use crate::errors::*;
use crate::keywords::NullScriptKeywords;
use crate::transpiler::{NullScriptTranspiler, OutputFormat, TranspileOptions};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;

#[derive(Parser)]
#[command(
    name = "nullc",
    version = "1.0.1",
    about = "NullScript transpiler - TypeScript with attitude",
    long_about = None,
    after_help = "Examples:
  nullc build src/                    # Transpile all .ns files in src/ to TypeScript
  nullc build src/ --js               # Transpile to JavaScript
  nullc run hello.ns                  # Run a NullScript file
  nullc check src/                    # Type-check NullScript files
  nullc keywords                       # Show all available keywords

Learn more at: https://github.com/kiron0/nullscript"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Transpile NullScript files to TypeScript or JavaScript
    Build(BuildArgs),
    /// Run a NullScript file directly
    Run(RunArgs),
    /// Type-check NullScript files using TypeScript
    Check(CheckArgs),
    /// Show all available NullScript keywords
    Keywords(KeywordsArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    /// Directory or file to transpile (.ns)
    pub path: PathBuf,

    /// Output directory
    #[arg(short = 'o', long = "outDir", default_value = "dist")]
    pub out_dir: PathBuf,

    /// Compile directly to JavaScript
    #[arg(long = "js")]
    pub js: bool,

    /// Transpile to TypeScript (default)
    #[arg(long = "ts")]
    pub ts: bool,

    /// Skip TypeScript type checking
    #[arg(long = "skip-type-check")]
    pub skip_type_check: bool,
}

#[derive(Args)]
pub struct RunArgs {
    /// NullScript file to run (.ns)
    pub file: PathBuf,

    /// Skip TypeScript type checking
    #[arg(long = "skip-type-check")]
    pub skip_type_check: bool,
}

#[derive(Args)]
pub struct CheckArgs {
    /// File or directory to type-check
    pub path: PathBuf,
}

#[derive(Args)]
pub struct KeywordsArgs {
    /// Show aliases for specific category
    #[arg(short = 'c', long = "category")]
    pub category: Option<String>,
}

pub struct CliHandler {
    transpiler: NullScriptTranspiler,
    keywords: NullScriptKeywords,
}

impl Default for CliHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CliHandler {
    pub fn new() -> Self {
        Self {
            transpiler: NullScriptTranspiler::new(),
            keywords: NullScriptKeywords::new(),
        }
    }

    pub async fn handle_command(&self, command: Commands) -> Result<(), NullScriptError> {
        match command {
            Commands::Build(args) => self.handle_build(args).await,
            Commands::Run(args) => self.handle_run(args).await,
            Commands::Check(args) => self.handle_check(args).await,
            Commands::Keywords(args) => self.handle_keywords(args),
        }
    }

    async fn handle_build(&self, args: BuildArgs) -> Result<(), NullScriptError> {
        let transpile_options = TranspileOptions {
            output_format: if args.js {
                OutputFormat::JavaScript
            } else {
                OutputFormat::TypeScript
            },
            skip_type_check: args.skip_type_check,
        };

        let metadata = fs::metadata(&args.path).await?;

        if metadata.is_dir() {
            let outputs = self
                .transpiler
                .build_directory(&args.path, &args.out_dir, &transpile_options)
                .await?;

            println!(
                "{}",
                format!("✅ Transpiled {} file(s) to {}", outputs.len(), args.out_dir.display())
                    .green()
            );

            for file in outputs {
                println!("{}   → {}", "".clear(), file.display().to_string().bright_black());
            }
        } else {
            let output_ext = if args.js { "js" } else { "ts" };
            let output_path = args.out_dir.join(
                args.path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    + "." + output_ext,
            );

            match transpile_options.output_format {
                OutputFormat::JavaScript => {
                    self.transpiler
                        .transpile_to_js(&args.path, &output_path, &transpile_options)
                        .await?;
                }
                OutputFormat::TypeScript => {
                    self.transpiler
                        .transpile_file(&args.path, &output_path, &transpile_options)
                        .await?;
                }
            }

            println!(
                "{}",
                format!("✅ Transpiled {} → {}", args.path.display(), output_path.display())
                    .green()
            );
        }

        Ok(())
    }

    async fn handle_run(&self, args: RunArgs) -> Result<(), NullScriptError> {
        println!("{}", "🚀 Running NullScript...".cyan());

        let temp_js = args.file.with_extension("temp.js");
        let transpile_options = TranspileOptions {
            output_format: OutputFormat::JavaScript,
            skip_type_check: args.skip_type_check,
        };

        self.transpiler
            .transpile_to_js(&args.file, &temp_js, &transpile_options)
            .await?;

        // Run the compiled JavaScript
        let output = if cfg!(target_os = "windows") {
            Command::new("node").arg(&temp_js).output()
        } else {
            Command::new("node").arg(&temp_js).output()
        };

        // Clean up temp file
        let _ = fs::remove_file(&temp_js).await;

        match output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("{}", "❌ Runtime error:".red());
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    std::process::exit(1);
                } else {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => {
                eprintln!("{} {}", "❌ Failed to run:".red(), e);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    async fn handle_check(&self, args: CheckArgs) -> Result<(), NullScriptError> {
        println!("{}", "🔍 Type-checking NullScript...".cyan());

        let temp_dir = PathBuf::from(".nullscript-check");
        let transpile_options = TranspileOptions {
            output_format: OutputFormat::TypeScript,
            skip_type_check: false,
        };

        // Transpile to .ts first
        self.transpiler
            .build_directory(&args.path, &temp_dir, &transpile_options)
            .await?;

        // Run tsc --noEmit
        let output = if cfg!(target_os = "windows") {
            Command::new("tsc")
                .args(&["--noEmit", "--project", &temp_dir.display().to_string()])
                .output()
        } else {
            Command::new("tsc")
                .args(&["--noEmit", "--project", &temp_dir.display().to_string()])
                .output()
        };

        // Clean up temp directory
        let _ = fs::remove_dir_all(&temp_dir).await;

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("{}", "✅ Type checking passed!".green());
                } else {
                    eprintln!("{}", "❌ Type checking failed:".red());
                    let error_output = String::from_utf8_lossy(&output.stdout);
                    let stderr_output = String::from_utf8_lossy(&output.stderr);

                    let combined_error = if !error_output.is_empty() {
                        error_output.to_string()
                    } else {
                        stderr_output.to_string()
                    };

                    let custom_error = parse_typescript_error(&combined_error, None);
                    eprintln!("{}", format_error(&custom_error).red());
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("{} {}", "❌ Type-check failed:".red(), e);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    fn handle_keywords(&self, args: KeywordsArgs) -> Result<(), NullScriptError> {
        if let Some(category_name) = args.category {
            if let Some(category) = self.keywords.get_category(&category_name) {
                println!("{}", format!("\n📋 {} Keywords:", category.title).cyan());
                println!("{}", "─".repeat(50).bright_black());

                for (alias, keyword) in &category.keywords {
                    println!(
                        "{}{}",
                        format!("  {:<15}", alias).yellow(),
                        format!("→ {}", keyword).white()
                    );
                }
            } else {
                eprintln!("{}", format!("❌ Unknown category: {}", category_name).red());
                println!("{}", "Available categories:".bright_black());
                for cat_name in self.keywords.get_category_names() {
                    println!("{}", format!("  - {}", cat_name).bright_black());
                }
                std::process::exit(1);
            }
        } else {
            println!("{}", "\n🎭 NullScript Keywords".cyan());
            println!("{}", "=".repeat(50).bright_black());

            for category in self.keywords.all_categories() {
                println!("{}", format!("\n📋 {}:", category.title).cyan());
                println!("{}", "─".repeat(30).bright_black());

                for (alias, keyword) in &category.keywords {
                    println!(
                        "{}{}",
                        format!("  {:<15}", alias).yellow(),
                        format!("→ {}", keyword).white()
                    );
                }
            }

            println!(
                "{}",
                "\n💡 Tip: Use 'nullc keywords --category <name>' to see specific categories"
                    .bright_black()
            );
            println!(
                "{}",
                format!(
                    "   Available categories: {}",
                    self.keywords.get_category_names().join(", ")
                )
                .bright_black()
            );
        }

        Ok(())
    }
}

pub async fn run() -> Result<(), NullScriptError> {
    let cli = Cli::parse();
    let handler = CliHandler::new();

    if let Err(e) = handler.handle_command(cli.command).await {
        eprintln!("{}", format_error(&e).red());
        std::process::exit(1);
    }

    Ok(())
}
