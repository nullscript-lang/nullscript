use crate::core::{NullScriptError, NullScriptKeywords};
use crate::compiler::NullScriptTranspiler;
use crate::core::types::{OutputFormat, TranspileOptions};
use crate::utils::commands::CommandUtils;
use crate::utils::strings::StringUtils;
use crate::utils::files::FileUtils;
use colored::Colorize;
use std::path::PathBuf;
use tokio::fs;

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

    pub async fn handle_build(&self, path: PathBuf, out_dir: PathBuf, js: bool, skip_type_check: bool) -> Result<(), NullScriptError> {
        self.show_build_info(&path, &out_dir)?;
        println!();

        let transpile_options = TranspileOptions {
            output_format: if js {
                OutputFormat::JavaScript
            } else {
                OutputFormat::TypeScript
            },
            skip_type_check,
        };

        let metadata = fs::metadata(&path).await?;

        if metadata.is_dir() {
            let outputs = self
                .transpiler
                .build_directory(&path, &out_dir, &transpile_options)
                .await?;

            println!(
                "{}",
                format!("✅ Transpiled {} file(s) to {}", outputs.len(), out_dir.display())
                    .green()
            );

            for file in outputs {
                println!("{}   → {}", "".clear(), file.display().to_string().bright_black());
            }
        } else {
            let output_ext = if js { "js" } else { "ts" };
            let output_path = out_dir.join(
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    + "." + output_ext,
            );

            match transpile_options.output_format {
                OutputFormat::JavaScript => {
                    self.transpiler
                        .transpile_to_js(&path, &output_path, &transpile_options)
                        .await?;
                }
                OutputFormat::TypeScript => {
                    self.transpiler
                        .transpile_file(&path, &output_path, &transpile_options)
                        .await?;
                }
            }

            println!(
                "{}",
                format!("✅ Transpiled {} → {}", path.display(), output_path.display())
                    .green()
            );
        }

        Ok(())
    }

    pub async fn handle_run(&self, file: PathBuf, skip_type_check: bool) -> Result<(), NullScriptError> {
        println!("{}", "🚀 Running NullScript...".cyan());

        let temp_js = file.with_extension("temp.js");
        let transpile_options = TranspileOptions {
            output_format: OutputFormat::JavaScript,
            skip_type_check,
        };

        self.transpiler
            .transpile_to_js(&file, &temp_js, &transpile_options)
            .await?;

        let output = CommandUtils::execute_node(&temp_js);

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

    pub async fn handle_check(&self, path: PathBuf) -> Result<(), NullScriptError> {
        println!("{}", "🔍 Type-checking NullScript...".cyan());

        let temp_dir = PathBuf::from(".nullscript-check");
        let transpile_options = TranspileOptions {
            output_format: OutputFormat::TypeScript,
            skip_type_check: false,
        };

        self.transpiler
            .build_directory(&path, &temp_dir, &transpile_options)
            .await?;

        let output = CommandUtils::execute_tsc_no_emit(&temp_dir.display().to_string());

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

                    let custom_error = crate::core::errors::parse_typescript_error(&combined_error, None);
                    eprintln!("{}", crate::core::errors::format_error(&custom_error).red());
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

    pub fn get_file_stats(&self, path: &PathBuf) -> Result<(usize, usize), NullScriptError> {
        let mut total_files = 0;
        let mut nullscript_files = 0;

        if path.is_dir() {
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    total_files += 1;
                    if FileUtils::is_nullscript_file(&entry.path().to_path_buf()) {
                        nullscript_files += 1;
                    }
                }
            }
        } else {
            total_files = 1;
            if FileUtils::is_nullscript_file(path) {
                nullscript_files = 1;
            }
        }

        Ok((total_files, nullscript_files))
    }

    pub fn show_build_info(&self, path: &PathBuf, out_dir: &PathBuf) -> Result<(), NullScriptError> {
        let (total_files, nullscript_files) = self.get_file_stats(path)?;

        let title = StringUtils::capitalize("build information");
        println!("{}", format!("📊 {}", title).cyan());
        println!("{}", "=".repeat(30).bright_black());
        println!("Input path: {}", path.display());
        println!("Output directory: {}", out_dir.display());
        println!("Total files: {}", total_files);
        println!("NullScript files: {}", nullscript_files);

        if nullscript_files > 0 {
            println!("Estimated output: {} TypeScript/JavaScript files", nullscript_files);
        }

        Ok(())
    }

    pub fn handle_keywords(&self, category: Option<String>) -> Result<(), NullScriptError> {
        if let Some(category_name) = category {
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
                "\n💡 Tip: Use 'nsc keywords --category <name>' to see specific categories"
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
