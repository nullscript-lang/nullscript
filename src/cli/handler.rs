use crate::core::NullScriptError;
use crate::compiler::NullScriptTranspiler;

use crate::utils::commands::CommandUtils;
use crate::utils::strings::StringUtils;
use crate::utils::files::FileUtils;
use colored::Colorize;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct CliHandler {
    transpiler: NullScriptTranspiler,
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
        }
    }

    pub async fn handle_build(&self, path: PathBuf, out_dir: PathBuf) -> Result<(), NullScriptError> {
        self.show_build_info(&path, &out_dir)?;
        println!();

        let metadata = fs::metadata(&path).await?;

        if metadata.is_dir() {
            let outputs = self
                .transpiler
                .build_directory(&path, &out_dir)
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
            let output_path = out_dir.join(
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    + ".js",
            );

            self.transpiler
                .transpile_to_js(&path, &output_path)
                .await?;

            println!(
                "{}",
                format!("✅ Transpiled {} → {}", path.display(), output_path.display())
                    .green()
            );
        }

        Ok(())
    }

    pub async fn handle_run(&self, file: PathBuf) -> Result<(), NullScriptError> {
        println!("{}", "🚀 Running NullScript...".cyan());

        let temp_js = file.with_extension("temp.js");

        self.transpiler
            .transpile_to_js(&file, &temp_js)
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
                    if FileUtils::is_nullscript_file(entry.path()) {
                        nullscript_files += 1;
                    }
                }
            }
        } else {
            total_files = 1;
            if FileUtils::is_nullscript_file(path.as_path()) {
                nullscript_files = 1;
            }
        }

        Ok((total_files, nullscript_files))
    }

    pub fn show_build_info(&self, path: &PathBuf, out_dir: &Path) -> Result<(), NullScriptError> {
        let (total_files, nullscript_files) = self.get_file_stats(path)?;

        let title = StringUtils::capitalize("build information");
        println!("{}", format!("📊 {}", title).cyan());
        println!("{}", "=".repeat(30).bright_black());
        println!("Input path: {}", path.display());
        println!("Output directory: {}", out_dir.display());
        println!("Total files: {}", total_files);
        println!("NullScript files: {}", nullscript_files);

        if nullscript_files > 0 {
            println!("Estimated output: {} JavaScript files", nullscript_files);
        }

        Ok(())
    }

    pub fn handle_keywords(&self, _category: Option<String>) -> Result<(), NullScriptError> {
        use crate::core::keywords::KEYWORDS;

        println!("{}", "\n🎭 NullScript Keywords".cyan());
        println!("{}", "=".repeat(50).bright_black());

        println!("{}", "\n📋 NullScript → JavaScript Keywords:".cyan());
        println!("{}", "─".repeat(40).bright_black());

        for (nullscript_keyword, js_keyword) in KEYWORDS.iter() {
            println!(
                "{}{}",
                format!("  {:<15}", nullscript_keyword).yellow(),
                format!("→ {}", js_keyword).white()
            );
        }

        println!(
            "{}",
            "\n💡 Tip: Use NullScript keywords in your .ns files, they will be transpiled to JavaScript"
                .bright_black()
        );

        Ok(())
    }
}
