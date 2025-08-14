use crate::errors::types::NullScriptError;
use crate::compiler::transpiler::NullScriptTranspiler;

use crate::common::commands::CommandUtils;
use crate::common::strings::StringUtils;
        use crate::common::files::FileUtils;
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
        use crate::language::keywords::KEYWORDS;

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

    pub fn handle_config(&self, args: crate::cli::commands::ConfigArgs) -> Result<(), NullScriptError> {
        use crate::config::loader::NullScriptConfig;
        use std::env;

        let current_dir = env::current_dir().map_err(|e| {
            NullScriptError::Io(e)
        })?;

        if args.generate {
            let config_path = current_dir.join("nsconfig.json");
            if config_path.exists() && !args.show {
                println!("{}", "⚠️  nsconfig.json already exists. Use --show to view it.".yellow());
                return Ok(());
            }

            NullScriptConfig::create_default_config(&current_dir).map_err(|e| {
                NullScriptError::Io(std::io::Error::other(e))
            })?;

            println!("{}", "✅ Generated nsconfig.json with default settings".green());
            return Ok(());
        }

        if args.validate {
            let config_path = current_dir.join("nsconfig.json");
            match NullScriptConfig::validate_file(&config_path) {
                Ok(_) => println!("{}", "✅ Configuration file is valid".green()),
                Err(e) => {
                    println!("{}", "❌ Configuration file is invalid:".red());
                    for (i, cause) in e.chain().enumerate() {
                        if i == 0 {
                            println!("   {}", cause);
                        } else {
                            println!("   Caused by: {}", cause);
                        }
                    }
                    return Err(NullScriptError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())));
                }
            }
            return Ok(());
        }

        if args.show {
            let config = NullScriptConfig::load_or_default(&current_dir);
            let config_json = config.to_pretty_json().map_err(|e| {
                NullScriptError::Io(std::io::Error::other(e))
            })?;

            println!("{}", "📋 Current NullScript Configuration".cyan());
            println!("{}", "=".repeat(40).bright_black());
            println!("{}", config_json);
            return Ok(());
        }

        // Default: show help for config command
        println!("{}", "🔧 NullScript Configuration Management".cyan());
        println!("{}", "=".repeat(40).bright_black());
        println!("Available options:");
        println!("  --generate    Generate default nsconfig.json");
        println!("  --show        Show current configuration");
        println!("  --validate    Validate configuration file");

        Ok(())
    }

    pub fn handle_init(&self, args: crate::cli::commands::InitArgs) -> Result<(), NullScriptError> {
        use crate::config::loader::NullScriptConfig;
        use std::{env, fs};

        let current_dir = env::current_dir().map_err(|e| {
            NullScriptError::Io(e)
        })?;

        let project_name = args.name.unwrap_or_else(|| {
            current_dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("nullscript-project")
                .to_string()
        });

        // Check if directory is empty
        if !args.force {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                let count = entries.count();
                if count > 0 {
                    println!("{}", "❌ Directory is not empty. Use --force to initialize anyway.".red());
                    return Ok(());
                }
            }
        }

        println!("{}", format!("🚀 Initializing NullScript project: {}", project_name).cyan());
        println!("{}", "=".repeat(50).bright_black());

        // Create directory structure
        let src_dir = current_dir.join("src");
        let tests_dir = current_dir.join("tests");

        fs::create_dir_all(&src_dir).map_err(NullScriptError::Io)?;
        fs::create_dir_all(&tests_dir).map_err(NullScriptError::Io)?;

        // Create nsconfig.json
        let config = NullScriptConfig::default();

        let config_path = current_dir.join("nsconfig.json");
        config.save_to_file(&config_path).map_err(|e| {
            NullScriptError::Io(std::io::Error::other(e))
        })?;

        // Create main.ns
        let main_content = format!(r#"// Welcome to NullScript! 🎭
// This is your main entry point

use {{ readFileSync }} from 'fs';

run main() {{
    speak.say("Hello from {}! 🎭");
    speak.say("NullScript is running successfully!");

    // Example: Fun keyword usage
    fixed greeting = "Welcome to NullScript";
    let isAwesome = yes;

    whatever (isAwesome) {{
        speak.say("🎉 " + greeting);
    }} otherwise {{
        speak.say("Something went wrong!");
    }}
}}

// Run the main function
main();
"#, project_name);

        let main_path = src_dir.join("main.ns");
        fs::write(&main_path, main_content).map_err(NullScriptError::Io)?;

        // Create example test
        let test_content = r#"// Example test file
// Run with: nsc test

use {{ describe, it, expect }} from 'test-framework';

describe("Basic NullScript functionality", () => {
    it("should use fun keywords", () => {
        fixed result = yes;
        expect(result).toBe(true);
    });

    it("should transpile correctly", () => {
        run testFunction() {
            return "Hello from NullScript!";
        }

        let message = testFunction();
        expect(message).toContain("NullScript");
    });
});
"#;

        let test_path = tests_dir.join("basic.test.ns");
        fs::write(&test_path, test_content).map_err(NullScriptError::Io)?;

        // Create .gitignore
        let gitignore_content = r#"# Build output
/dist/
/build/

# Dependencies
node_modules/
*.log

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# NullScript specific
.ns-cache/
*.ns.map
"#;

        let gitignore_path = current_dir.join(".gitignore");
        fs::write(&gitignore_path, gitignore_content).map_err(NullScriptError::Io)?;

        // Create package.json
        let package_json = serde_json::json!({
            "name": project_name,
            "version": "1.0.0",
            "description": "A NullScript project",
            "main": "dist/main.js",
            "scripts": {
                "build": "nsc build src/",
                "dev": "nsc dev src/",
                "start": "nsc run src/main.ns",
                "test": "nsc test tests/"
            },
            "keywords": ["nullscript", "javascript", "transpiler"],
            "license": "MIT"
        });

        let package_path = current_dir.join("package.json");
        fs::write(&package_path, serde_json::to_string_pretty(&package_json).unwrap())
            .map_err(NullScriptError::Io)?;

        println!("✅ Created directory structure:");
        println!("   📁 src/");
        println!("   📁 tests/");
        println!("   📄 src/main.ns");
        println!("   📄 tests/basic.test.ns");
        println!("   📄 nsconfig.json");
        println!("   📄 package.json");
        println!("   📄 .gitignore");

        println!("\n{}", "🎉 Project initialized successfully!".green());
        println!("\n{}", "Next steps:".cyan());
        println!("  1. Build your project: {}", "nsc build src/".yellow());
        println!("  2. Run your project:  {}", "nsc run src/main.ns".yellow());
        println!("  3. Start development: {}", "nsc dev src/".yellow());

        Ok(())
    }

    pub fn handle_complete(&self, args: crate::cli::commands::CompleteArgs) -> Result<(), NullScriptError> {
        use crate::analysis::completion::{LanguageServer, CompletionContext};
        use std::fs;

        if !args.file.exists() {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", args.file.display())
            )));
        }

        let content = fs::read_to_string(&args.file).map_err(NullScriptError::Io)?;
        let lines: Vec<&str> = content.lines().collect();

        if args.line == 0 || args.line > lines.len() as u32 {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid line number: {}", args.line)
            )));
        }

        let line_index = (args.line - 1) as usize;
        let line = lines[line_index];

        if args.column == 0 || args.column > line.len() as u32 + 1 {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid column number: {}", args.column)
            )));
        }

        // Calculate position in the entire text
        let mut position = 0;
        for (i, line_text) in lines.iter().enumerate() {
            if i == line_index {
                position += (args.column - 1) as usize;
                break;
            }
            position += line_text.len() + 1; // +1 for newline
        }

        let context = CompletionContext {
            text: content.clone(),
            position,
        };

        let language_server = LanguageServer::new();
        let completions = language_server.get_completions(context.clone());
        let signature_help = language_server.get_signature_help(context.clone());
        let diagnostics = language_server.get_diagnostics(&content);

        match args.format.as_str() {
            "json" => {
                let result = serde_json::json!({
                    "completions": completions,
                    "signatureHelp": signature_help,
                    "diagnostics": diagnostics
                });
                println!("{}", serde_json::to_string_pretty(&result).map_err(NullScriptError::Json)?);
            }
            "text" => {
                println!("{}", "🎯 Auto-Completion Results".cyan());
                println!("{}", "=".repeat(30).bright_black());

                if !completions.is_empty() {
                    println!("\n{}", "📝 Completions:".yellow());
                    for (i, completion) in completions.iter().take(10).enumerate() {
                        let kind_icon = match completion.kind {
                            crate::analysis::completion::CompletionKind::Keyword => "🔤",
                            crate::analysis::completion::CompletionKind::Function => "🔧",
                            crate::analysis::completion::CompletionKind::Method => "⚙️",
                            crate::analysis::completion::CompletionKind::Snippet => "📄",
                            _ => "📋",
                        };

                        println!("  {}. {} {} - {}",
                            i + 1,
                            kind_icon,
                            completion.label.green(),
                            completion.detail.as_ref().unwrap_or(&"".to_string()).bright_black()
                        );
                    }
                    if completions.len() > 10 {
                        println!("  ... and {} more", completions.len() - 10);
                    }
                }

                if let Some(ref sig_help) = signature_help {
                    println!("\n{}", "📋 Signature Help:".yellow());
                    for signature in &sig_help.signatures {
                        println!("  {}", signature.label.cyan());
                        if let Some(doc) = &signature.documentation {
                            println!("    {}", doc.bright_black());
                        }
                    }
                }

                if !diagnostics.is_empty() {
                    println!("\n{}", "⚠️  Diagnostics:".yellow());
                    for diagnostic in &diagnostics {
                        let severity_icon = match diagnostic.severity {
                            crate::analysis::completion::DiagnosticSeverity::Error => "❌",
                            crate::analysis::completion::DiagnosticSeverity::Warning => "⚠️",
                            crate::analysis::completion::DiagnosticSeverity::Information => "ℹ️",
                            crate::analysis::completion::DiagnosticSeverity::Hint => "💡",
                        };

                        println!("  {} Line {}: {}",
                            severity_icon,
                            diagnostic.range.start.line + 1,
                            diagnostic.message
                        );
                    }
                }

                if completions.is_empty() && signature_help.is_none() && diagnostics.is_empty() {
                    println!("No completions, signature help, or diagnostics available at this position.");
                }
            }
            _ => {
                return Err(NullScriptError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Unsupported format: {}", args.format)
                )));
            }
        }

        Ok(())
    }

        pub async fn handle_dev(&self, args: crate::cli::commands::DevArgs) -> Result<(), NullScriptError> {
        use crate::development::watcher::TerminalDevCommand;
        use crate::config::loader::NullScriptConfig;
        use std::env;

        let current_dir = env::current_dir().map_err(NullScriptError::Io)?;
        let config = NullScriptConfig::load_or_default(&current_dir);

        if !args.path.exists() {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Watch directory does not exist: {}", args.path.display())
            )));
        }

        if !args.watch {
            println!("{}", "ℹ️  Use --watch flag to enable file watching".yellow());
            println!("Example: nsc dev src --watch");
            return Ok(());
        }

        // Start terminal development mode
        let mut dev_command = TerminalDevCommand::new(config);
        let watch_paths = vec![args.path];

        dev_command.start(watch_paths, args.run_on_save).await.map_err(|e| {
            NullScriptError::Io(std::io::Error::other(e.to_string()))
        })?;

        Ok(())
    }



    pub async fn handle_analyze(&self, args: crate::cli::commands::AnalyzeArgs) -> Result<(), NullScriptError> {
        use crate::analysis::analytics::PerformanceAnalyzer;
        use crate::config::loader::NullScriptConfig;
        use crate::compiler::transpiler::NullScriptTranspiler;
        use std::{env, fs};
        use walkdir::WalkDir;

        let current_dir = env::current_dir().map_err(NullScriptError::Io)?;
        let _config = NullScriptConfig::load_or_default(&current_dir);

        // Use CLI arguments for performance analysis (not stored in simplified config)
        let report_format = args.format;
        let _bundle_size_limit = args.bundle_size_limit;
        let _build_time_budget = args.build_time_budget;

        if !args.path.exists() {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Input directory does not exist: {}", args.path.display())
            )));
        }

        println!("{}", "📊 Starting Performance Analysis".cyan());
        println!("{}", "=".repeat(40).bright_black());
        println!("📁 Analyzing: {}", args.path.display());
        println!("📄 Report format: {}", report_format);
        println!("📂 Output directory: {}", args.output.display());
        println!("{}", "=".repeat(40).bright_black());

        let mut analyzer = PerformanceAnalyzer::new();
        let transpiler = NullScriptTranspiler::new();

        analyzer.start_build();

        // Find all .ns files
        let mut file_count = 0;
        for entry in WalkDir::new(&args.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "ns"))
        {
            let file_path = entry.path().to_path_buf();
            println!("📄 Processing: {}", file_path.display());

            analyzer.start_file(file_path.clone());

            // Read and transpile file
            let input_content = fs::read_to_string(&file_path).map_err(NullScriptError::Io)?;

            match transpiler.transpile(&input_content) {
                Ok(output_content) => {
                    analyzer.finish_file(file_path, &input_content, &output_content)
                        .map_err(|e| NullScriptError::Io(std::io::Error::other(e.to_string())))?;
                }
                Err(e) => {
                    eprintln!("⚠️  Transpilation error for {}: {}", file_path.display(), e);
                    // Still record the file with empty output for analysis
                    analyzer.finish_file(file_path, &input_content, "")
                        .map_err(|e| NullScriptError::Io(std::io::Error::other(e.to_string())))?;
                }
            }

            file_count += 1;
        }

        if file_count == 0 {
            println!("⚠️  No .ns files found in {}", args.path.display());
            return Ok(());
        }

        // Finish analysis and generate report
        let metrics = analyzer.finish_build()
            .map_err(|e| NullScriptError::Io(std::io::Error::other(e.to_string())))?;

        // Print summary to console
        self.print_analysis_summary(&metrics);

        // Save detailed report
        analyzer.save_report(&metrics, &args.output, &report_format).await
            .map_err(|e| NullScriptError::Io(std::io::Error::other(e.to_string())))?;

        println!("\n{}", "✅ Performance analysis completed successfully!".green());
        println!("📊 Report saved to: {}", args.output.display());

        Ok(())
    }

    fn print_analysis_summary(&self, metrics: &crate::analysis::analytics::PerformanceMetrics) {
        println!("\n{}", "📊 Performance Summary".cyan());
        println!("{}", "=".repeat(30).bright_black());

        println!("🕒 Total Build Time: {}ms", metrics.total_build_time_ms);
        println!("📦 Bundle Size: {} bytes ({} files)",
            metrics.bundle_analysis.total_output_size,
            metrics.bundle_analysis.file_count
        );
        println!("🗜️  Compression Ratio: {:.2}%",
            metrics.bundle_analysis.compression_ratio * 100.0
        );

        // Show largest files
        if !metrics.bundle_analysis.largest_files.is_empty() {
            println!("\n{}", "📄 Largest Files:".yellow());
            for (i, file) in metrics.bundle_analysis.largest_files.iter().take(5).enumerate() {
                println!("  {}. {} - {} bytes ({:.1}%)",
                    i + 1,
                    file.file_path.display(),
                    file.size_bytes,
                    file.percentage_of_total
                );
            }
        }

        // Show optimization suggestions
        if !metrics.bundle_analysis.optimization_suggestions.is_empty() {
            println!("\n{}", "💡 Top Optimization Suggestions:".yellow());
            for (i, suggestion) in metrics.bundle_analysis.optimization_suggestions.iter().take(3).enumerate() {
                                    let priority_icon = match suggestion.priority {
                        crate::analysis::analytics::OptimizationPriority::High => "🔴",
                        crate::analysis::analytics::OptimizationPriority::Medium => "🟡",
                        _ => "🟢",
                    };
                println!("  {}. {} {:?}: {} (Save {} bytes)",
                    i + 1,
                    priority_icon,
                    suggestion.suggestion_type,
                    suggestion.description,
                    suggestion.potential_savings_bytes
                );
            }
        }

        // Show duplicates
        if !metrics.bundle_analysis.duplicate_detection.is_empty() {
            println!("\n{}", "🔍 Duplicate Files Detected:".yellow());
            for duplicate in &metrics.bundle_analysis.duplicate_detection {
                println!("  {} duplicate files ({} bytes each)",
                    duplicate.files.len(),
                    duplicate.size_bytes
                );
            }
        }
    }

    pub async fn handle_debug(&self, args: crate::cli::commands::DebugArgs) -> Result<(), NullScriptError> {
        use crate::development::debugger::NullScriptDebugger;
        use crate::config::loader::NullScriptConfig;
        use std::env;

        let current_dir = env::current_dir().map_err(NullScriptError::Io)?;
        let config = NullScriptConfig::load_or_default(&current_dir);

        // Enable profiling if requested
        let _enable_profiling = args.profile;

        if !args.file.exists() {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", args.file.display())
            )));
        }

        if args.file.extension().is_none_or(|ext| ext != "ns") {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Debug command only works with .ns files"
            )));
        }

        println!("{}", "🐛 Starting NullScript Debugger".cyan());
        println!("{}", "=".repeat(30).bright_black());
        println!("📄 File: {}", args.file.display());
        if let Some(bp) = args.breakpoint {
            println!("🔴 Initial breakpoint: line {}", bp);
        }
        if args.profile {
            println!("📊 Performance profiling: enabled");
        }
        println!("{}", "=".repeat(30).bright_black());

        let mut debugger = NullScriptDebugger::new(config);

        // Set initial breakpoint if specified
        if let Some(line) = args.breakpoint {
            println!("Setting initial breakpoint at line {}", line);
        }

        // Start debug session
        debugger.start_debug_session(args.file).await.map_err(|e| {
            NullScriptError::Io(std::io::Error::other(e.to_string()))
        })?;

        println!("✅ Debug session completed");
        Ok(())
    }

    pub async fn handle_convert(&self, args: crate::cli::commands::ConvertArgs) -> Result<(), NullScriptError> {
        use crate::compiler::reverse_transpiler::ReverseTranspiler;
        use std::fs;

        if !args.file.exists() {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", args.file.display())
            )));
        }

        let valid_extensions = ["js", "mjs", "ts"];
        let is_valid = args.file.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| valid_extensions.contains(&ext))
            .unwrap_or(false);

        if !is_valid {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Convert command only works with .js, .mjs, or .ts files"
            )));
        }

        println!("{}", "🔄 Converting JavaScript to NullScript".cyan());
        println!("{}", "=".repeat(40).bright_black());
        println!("📄 Input: {}", args.file.display());

        let input_content = fs::read_to_string(&args.file).map_err(NullScriptError::Io)?;
        let transpiler = ReverseTranspiler::new();

        let converted_content = transpiler.reverse_transpile(&input_content).map_err(|e| {
            NullScriptError::Io(std::io::Error::other(e.to_string()))
        })?;

        // Format the code if requested
        let final_content = if args.format {
            self.format_nullscript_code(&converted_content)
        } else {
            converted_content
        };

        // Determine output path
        let output_path = args.output.unwrap_or_else(|| {
            args.file.with_extension("ns")
        });

        fs::write(&output_path, &final_content).map_err(NullScriptError::Io)?;

        println!("📄 Output: {}", output_path.display());
        println!("✅ Conversion completed successfully!");

        // Show conversion report if requested
        if args.report {
            let report = transpiler.analyze_conversion_quality(&input_content, &final_content);
            report.print_report();
        }

        Ok(())
    }

    pub async fn handle_analytics(&self, args: crate::cli::commands::AnalyticsArgs) -> Result<(), NullScriptError> {
        use std::{env, fs};
        use walkdir::WalkDir;

        let current_dir = if args.path == Path::new(".") {
            env::current_dir().map_err(NullScriptError::Io)?
        } else {
            args.path.clone()
        };

        if !current_dir.exists() {
            return Err(NullScriptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory not found: {}", current_dir.display())
            )));
        }

        println!("{}", "📊 NullScript Development Analytics".cyan());
        println!("{}", "=".repeat(40).bright_black());
        println!("📁 Project: {}", current_dir.display());
        println!("📅 Analysis period: {} days", args.days);
        println!("{}", "=".repeat(40).bright_black());

        // Collect project statistics
        let mut total_ns_files = 0;
        let mut total_js_files = 0;
        let mut total_lines = 0;
        let mut total_size = 0;
        let mut keyword_usage = std::collections::HashMap::new();
        let mut largest_files = Vec::new();

        for entry in WalkDir::new(&current_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                match extension {
                    "ns" => {
                        total_ns_files += 1;
                        if let Ok(content) = fs::read_to_string(path) {
                            let lines = content.lines().count();
                            let size = content.len();
                            total_lines += lines;
                            total_size += size;

                            // Track largest files
                            largest_files.push((path.to_path_buf(), size, lines));

                            // Analyze keyword usage
                            self.analyze_keyword_usage(&content, &mut keyword_usage);
                        }
                    }
                    "js" | "mjs" => {
                        total_js_files += 1;
                    }
                    _ => {}
                }
            }
        }

        // Sort largest files by size
        largest_files.sort_by(|a, b| b.1.cmp(&a.1));
        largest_files.truncate(10);

        // Display analytics
        println!("\n📈 Project Overview");
        println!("{}", "─".repeat(20));
        println!("📄 NullScript files: {}", total_ns_files);
        println!("📄 JavaScript files: {}", total_js_files);
        println!("📏 Total lines of code: {}", total_lines);
        println!("💾 Total code size: {} bytes", total_size);

        if total_ns_files > 0 {
            println!("📊 Average file size: {} bytes", total_size / total_ns_files);
            println!("📊 Average lines per file: {}", total_lines / total_ns_files);
        }

        // Show keyword usage
        if !keyword_usage.is_empty() {
            println!("\n🔤 Most Used Keywords");
            println!("{}", "─".repeat(22));
            let mut keyword_vec: Vec<_> = keyword_usage.iter().collect();
            keyword_vec.sort_by(|a, b| b.1.cmp(a.1));

            for (keyword, count) in keyword_vec.iter().take(10) {
                println!("  {:15} → {} uses", keyword, count);
            }
        }

        // Show largest files
        if !largest_files.is_empty() {
            println!("\n📄 Largest Files");
            println!("{}", "─".repeat(15));
            for (i, (path, size, lines)) in largest_files.iter().enumerate() {
                let relative_path = path.strip_prefix(&current_dir).unwrap_or(path);
                println!("  {}. {} ({} bytes, {} lines)",
                    i + 1,
                    relative_path.display(),
                    size,
                    lines
                );
            }
        }

        // Development insights
        println!("\n💡 Development Insights");
        println!("{}", "─".repeat(23));

        if total_ns_files == 0 {
            println!("  • No NullScript files found - consider converting JS files");
        } else if total_js_files > total_ns_files {
            println!("  • More JS files than NS files - consider converting remaining JS files");
        } else {
            println!("  • Good NullScript adoption in this project!");
        }

        if total_lines > 0 {
            let avg_complexity = total_lines as f64 / total_ns_files as f64;
            if avg_complexity > 100.0 {
                println!("  • Consider breaking down large files for better maintainability");
            }
        }

        // Project health assessment
        let health_score = self.calculate_project_health(total_ns_files, total_js_files, total_lines);
        println!("\n🏥 Project Health Score: {:.1}/10", health_score);

        if health_score >= 8.0 {
            println!("✅ Excellent NullScript project!");
        } else if health_score >= 6.0 {
            println!("🟡 Good project with room for improvement");
        } else {
            println!("🟠 Consider optimizing your NullScript usage");
        }

        Ok(())
    }

    fn format_nullscript_code(&self, code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut formatted_lines = Vec::new();
        let mut indent_level: i32 = 0;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                formatted_lines.push(String::new());
                continue;
            }

            // Decrease indent for closing braces
            if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
            }

            // Add current line with proper indentation
            let indent = "    ".repeat(indent_level as usize);
            formatted_lines.push(format!("{}{}", indent, trimmed));

            // Increase indent for opening braces
            if trimmed.ends_with('{') {
                indent_level += 1;
            }
        }

        formatted_lines.join("\n")
    }

    fn analyze_keyword_usage(&self, content: &str, keyword_usage: &mut std::collections::HashMap<String, usize>) {
        use crate::language::keywords::KEYWORDS;

        for (ns_keyword, _) in KEYWORDS.iter() {
            let count = content.matches(ns_keyword).count();
            if count > 0 {
                *keyword_usage.entry(ns_keyword.to_string()).or_insert(0) += count;
            }
        }
    }

    fn calculate_project_health(&self, ns_files: usize, js_files: usize, total_lines: usize) -> f64 {
        let mut score = 5.0; // Base score

        // Prefer NullScript files
        if ns_files > 0 {
            let ns_ratio = ns_files as f64 / (ns_files + js_files) as f64;
            score += ns_ratio * 3.0; // Up to +3 points for high NS ratio
        }

        // Reasonable project size
        if total_lines > 100 && total_lines < 10000 {
            score += 1.0; // +1 for reasonable size
        }

        // Bonus for having any NS files
        if ns_files > 0 {
            score += 1.0;
        }

        score.min(10.0) // Cap at 10
    }

    pub async fn handle_analyze_clean(&self, args: crate::cli::commands::AnalyzeCleanArgs) -> Result<(), NullScriptError> {
        use std::io::{self, Write};
        use tokio::fs;

        println!("{}", "🧹 NullScript Analysis Cleanup".cyan());
        println!("{}", "=".repeat(35).bright_black());

        if !args.reports_dir.exists() {
            println!("{}", format!("ℹ️  Reports directory '{}' does not exist - nothing to clean.", args.reports_dir.display()).yellow());
            return Ok(());
        }

        // Check if directory contains any files
        let mut entries = fs::read_dir(&args.reports_dir).await.map_err(NullScriptError::Io)?;
        let mut file_count = 0;
        let mut total_size = 0u64;

        while let Some(entry) = entries.next_entry().await.map_err(NullScriptError::Io)? {
            if entry.file_type().await.map_err(NullScriptError::Io)?.is_file() {
                if let Ok(metadata) = entry.metadata().await {
                    total_size += metadata.len();
                    file_count += 1;
                }
            }
        }

        if file_count == 0 {
            println!("{}", format!("ℹ️  No files found in '{}' - nothing to clean.", args.reports_dir.display()).yellow());
            return Ok(());
        }

        println!("📂 Directory: {}", args.reports_dir.display());
        println!("📄 Files to remove: {}", file_count);
        println!("💾 Total size: {} bytes", total_size);
        println!("{}", "=".repeat(35).bright_black());

        // Ask for confirmation unless --force is used
        if !args.force {
            print!("{}", "⚠️  Are you sure you want to remove all analysis reports? (y/N): ".yellow());
            io::stdout().flush().map_err(NullScriptError::Io)?;

            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(NullScriptError::Io)?;
            let response = input.trim().to_lowercase();

            if response != "y" && response != "yes" {
                println!("{}", "❌ Operation cancelled.".red());
                return Ok(());
            }
        }

        // Remove all files in the reports directory
        println!("{}", "🗑️  Removing analysis reports...".cyan());

        let mut entries = fs::read_dir(&args.reports_dir).await.map_err(NullScriptError::Io)?;
        let mut removed_count = 0;

        while let Some(entry) = entries.next_entry().await.map_err(NullScriptError::Io)? {
            let entry_path = entry.path();
            if entry.file_type().await.map_err(NullScriptError::Io)?.is_file() {
                if let Err(e) = fs::remove_file(&entry_path).await {
                    eprintln!("{}", format!("⚠️  Failed to remove {}: {}", entry_path.display(), e).yellow());
                } else {
                    println!("   ✅ Removed: {}", entry_path.file_name().unwrap_or_default().to_string_lossy());
                    removed_count += 1;
                }
            }
        }

        // Remove the directory if it's empty and it's the default reports directory
        if args.reports_dir.file_name().unwrap_or_default() == "reports" {
            if let Err(e) = fs::remove_dir(&args.reports_dir).await {
                // It's okay if this fails (directory might not be empty or might have subdirs)
                println!("{}", format!("ℹ️  Note: Could not remove directory '{}': {}", args.reports_dir.display(), e).bright_black());
            } else {
                println!("   ✅ Removed directory: {}", args.reports_dir.display());
            }
        }

        println!("{}", "=".repeat(35).bright_black());
        println!("{}", format!("✅ Cleanup completed! Removed {} file(s).", removed_count).green());

        if removed_count > 0 {
            println!("{}", "💡 Run 'nsc analyze src/' to generate new reports.".bright_black());
        }

        Ok(())
    }
}
