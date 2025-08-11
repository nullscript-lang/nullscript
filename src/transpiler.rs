use crate::errors::*;
use crate::keywords::NullScriptKeywords;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct TranspileOptions {
    pub output_format: OutputFormat,
    pub skip_type_check: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    TypeScript,
    JavaScript,
}

impl Default for TranspileOptions {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::TypeScript,
            skip_type_check: false,
        }
    }
}

pub struct NullScriptTranspiler {
    keywords: NullScriptKeywords,
}

impl Default for NullScriptTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

impl NullScriptTranspiler {
    pub fn new() -> Self {
        Self {
            keywords: NullScriptKeywords::new(),
        }
    }

    /// Validate NullScript syntax before transpiling
    pub fn validate_syntax(&self, source: &str, file_path: Option<&Path>) -> Result<(), NullScriptError> {
        let lines: Vec<&str> = source.split('\n').collect();

        for (i, line) in lines.iter().enumerate() {
            let line = line.trim();
            let line_number = i as u32 + 1;

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") || line.starts_with("/*") {
                continue;
            }

            // Check for invalid keywords (using standard TS/JS instead of NullScript)
            let invalid_patterns = vec![
                (r"\b(function\s+\w+\s*\()", "using 'function' instead of 'feels'"),
                (r"\b(const\s+\w+)", "using 'const' instead of 'definitely'"),
                (r"\b(let\s+\w+)", "using 'let' instead of 'maybe'"),
                (r"\b(var\s+\w+)", "using 'var' instead of 'mayhap'"),
                (r"\b(if\s*\()", "using 'if' instead of 'checkthis'"),
                (r"\b(else\s+)", "using 'else' instead of 'orelse'"),
                (r"\b(return\s+)", "using 'return' instead of 'pls'"),
                (r"\b(true)\b", "using 'true' instead of 'fr'"),
                (r"\b(false)\b", "using 'false' instead of 'cap'"),
                (r"\b(null)\b", "using 'null' instead of 'nocap'"),
                (r"\b(undefined)\b", "using 'undefined' instead of 'ghost'"),
                (r"\b(interface\s+\w+)", "using 'interface' instead of 'vibes'"),
                (r"\b(type\s+\w+)", "using 'type' instead of 'vibe'"),
                (r"\b(class\s+\w+)", "using 'class' instead of 'bigbrain'"),
                (r"\b(try\s*\{)", "using 'try' instead of 'oops' or 'oop'"),
                (r"\b(catch\s*\()", "using 'catch' instead of 'mybad'"),
                (r"\b(finally\s*\{)", "using 'finally' instead of 'anyway'"),
            ];

            for (pattern, _description) in invalid_patterns {
                let regex = Regex::new(pattern)?;
                if regex.is_match(line) {
                    let message = format!(
                        "Invalid syntax on line {}: You're using standard TypeScript/JavaScript syntax instead of NullScript keywords.\n💡 Run 'nullc keywords' to see the correct NullScript syntax.",
                        line_number
                    );
                    return Err(NullScriptError::Syntax(
                        NullScriptSyntaxError::with_location(
                            message,
                            file_path.map(|p| p.to_path_buf()),
                            Some(line_number),
                            None,
                        )
                    ));
                }
            }

            // Check for unknown keywords
            let unknown_keyword_regex = Regex::new(r"^(\w+)\s+\w+\s*=")?;
            if let Some(captures) = unknown_keyword_regex.captures(line) {
                if let Some(keyword_match) = captures.get(1) {
                    let keyword = keyword_match.as_str();
                    let all_keywords = self.keywords.get_all_keywords();
                    let valid_keywords = ["export", "import", "from", "as"];

                    if !all_keywords.contains_key(keyword) && !valid_keywords.contains(&keyword) {
                        let message = format!(
                            "Unknown keyword '{}' on line {}.\n💡 Use valid NullScript keywords. Run 'nullc keywords' to see all available options.",
                            keyword, line_number
                        );
                        return Err(NullScriptError::Syntax(
                            NullScriptSyntaxError::with_location(
                                message,
                                file_path.map(|p| p.to_path_buf()),
                                Some(line_number),
                                None,
                            )
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Transpile NullScript source code to TypeScript/JavaScript
    pub fn transpile(&self, source: &str, _options: &TranspileOptions) -> Result<String, NullScriptError> {
        let mut output = source.to_string();

        // Handle function declarations first
        for (alias, ts_keyword) in self.keywords.get_function_keywords() {
            if alias.contains("async") {
                let regex = Regex::new(&format!(r"\b{}\s+([a-zA-Z_$][\w$]*)", regex::escape(alias)))?;
                output = regex.replace_all(&output, format!("{} $1", ts_keyword)).to_string();
            } else {
                // Handle function declarations more carefully
                let regex = Regex::new(&format!(
                    r"\b{}\s+([a-zA-Z_$][\w$]*)\s*(?:<[^>]*>)?\s*\(",
                    regex::escape(alias)
                ))?;

                                output = regex.replace_all(&output, |caps: &regex::Captures| {
                    let _function_name = &caps[1];
                    let lines: Vec<&str> = output.split('\n').collect();
                    let match_pos = caps.get(0).unwrap().start();
                    let current_line_index = output[..match_pos].matches('\n').count();

                    if current_line_index < lines.len() {
                        let current_line = lines[current_line_index];
                        let indent = current_line.chars()
                            .take_while(|&c| c.is_whitespace())
                            .collect::<String>();

                        if !indent.is_empty() {
                            // Method declaration (indented)
                            caps[0].replace(alias, "").trim_start().to_string()
                        } else {
                            // Top-level function declaration
                            format!("{} {}", ts_keyword, caps[0].replace(alias, "").trim_start())
                        }
                    } else {
                        format!("{} {}", ts_keyword, caps[0].replace(alias, "").trim_start())
                    }
                }).to_string();

                // Handle anonymous functions
                let anon_regex = Regex::new(&format!(r"\b{}\s*\(", regex::escape(alias)))?;
                output = anon_regex.replace_all(&output, format!("{}(", ts_keyword)).to_string();
            }
        }

        // Handle regular keywords (excluding function keywords)
        let all_keywords = self.keywords.get_all_keywords();
        for (alias, ts_keyword) in all_keywords {
            if alias == "feels" || alias == "feels async" {
                continue; // Already handled above
            }

            if alias == "remove" {
                // Special handling for delete operator
                let regex = Regex::new(r"\bremove\s+([a-zA-Z_$][\w$]*(?:\.[a-zA-Z_$][\w$]*)*(?:\[[^\]]+\])?)\b")?;
                output = regex.replace_all(&output, "delete $1").to_string();
            } else {
                let regex = Regex::new(&format!(r"\b{}\b", regex::escape(&alias)))?;
                output = regex.replace_all(&output, ts_keyword.as_str()).to_string();
            }
        }

        // Handle multi-word keywords
        for (alias, ts_keyword) in self.keywords.get_multi_word_keywords() {
            let regex = Regex::new(&format!(r"\b{}\s+", regex::escape(alias)))?;
            output = regex.replace_all(&output, format!("{} ", ts_keyword)).to_string();
        }

        Ok(output)
    }

    /// Transpile a single file
    pub async fn transpile_file(
        &self,
        input_path: &Path,
        output_path: &Path,
        options: &TranspileOptions,
    ) -> Result<String, NullScriptError> {
        let source = fs::read_to_string(input_path).await?;

        // Validate syntax before transpiling
        self.validate_syntax(&source, Some(input_path))?;

        let transpiled = self.transpile(&source, options)?;

        // Create output directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(output_path, &transpiled).await?;

        Ok(transpiled)
    }

        /// Transpile to JavaScript via TypeScript
    pub async fn transpile_to_js(
        &self,
        ns_path: &Path,
        js_path: &Path,
        options: &TranspileOptions,
    ) -> Result<(), NullScriptError> {
        // Create a temporary directory for compilation
        let temp_dir = std::env::temp_dir().join("nullscript-temp");
        fs::create_dir_all(&temp_dir).await?;

        let ts_filename = ns_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("temp")
            .to_string() + ".ts";

        let temp_ts_path = temp_dir.join(&ts_filename);

        // First transpile to TypeScript in temp directory
        let ts_options = TranspileOptions {
            output_format: OutputFormat::TypeScript,
            ..options.clone()
        };

        match self.transpile_file(ns_path, &temp_ts_path, &ts_options).await {
            Ok(_) => {},
            Err(e) => {
                // Clean up temp directory
                let _ = fs::remove_dir_all(&temp_dir).await;
                return Err(e);
            }
        }

        let tsconfig_path = temp_dir.join("tsconfig.json");
        let js_filename = ns_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("temp")
            .to_string() + ".js";
        let temp_js_path = temp_dir.join(&js_filename);

        let tsconfig = serde_json::json!({
            "compilerOptions": {
                "target": "ES2022",
                "module": "ES2022",
                "moduleResolution": "node",
                "outDir": ".",
                "esModuleInterop": true,
                "allowSyntheticDefaultImports": true,
                "skipLibCheck": true,
                "noEmit": false,
            },
            "include": [ts_filename]
        });

        // Create temporary tsconfig.json
        fs::write(&tsconfig_path, serde_json::to_string_pretty(&tsconfig)?).await?;

        // Run TypeScript compiler in the temp directory
        let tsc_args = if options.skip_type_check {
            vec!["--noCheck", "--project", "tsconfig.json"]
        } else {
            vec!["--project", "tsconfig.json"]
        };

        let tsc_output = if cfg!(target_os = "windows") {
            Command::new("tsc")
                .args(&tsc_args)
                .current_dir(&temp_dir)
                .output()
        } else {
            Command::new("tsc")
                .args(&tsc_args)
                .current_dir(&temp_dir)
                .output()
        };

        let result = match tsc_output {
            Ok(output) => {
                if !output.status.success() {
                    let error_output = String::from_utf8_lossy(&output.stdout);
                    let stderr_output = String::from_utf8_lossy(&output.stderr);

                    let combined_error = if !error_output.is_empty() {
                        error_output.to_string()
                    } else if !stderr_output.is_empty() {
                        stderr_output.to_string()
                    } else {
                        "TypeScript compilation failed".to_string()
                    };

                    Err(parse_typescript_error(&combined_error, Some(ns_path.to_path_buf())))
                } else {
                    // Move the generated JS file to the target location
                    match fs::metadata(&temp_js_path).await {
                        Ok(_) => {
                            // Create output directory if needed
                            if let Some(parent) = js_path.parent() {
                                fs::create_dir_all(parent).await?;
                            }
                            fs::copy(&temp_js_path, js_path).await?;
                            Ok(())
                        }
                        Err(_) => {
                            Err(NullScriptError::Transpile(NullScriptTranspileError::with_location(
                                "JavaScript file was not generated by TypeScript compiler".to_string(),
                                Some(ns_path.to_path_buf()),
                                None,
                                None,
                            )))
                        }
                    }
                }
            }
            Err(e) => Err(NullScriptError::Io(e)),
        };

        // Clean up temporary directory
        let _ = fs::remove_dir_all(&temp_dir).await;

        result
    }

    /// Build an entire directory of .ns files
    pub async fn build_directory(
        &self,
        input_dir: &Path,
        output_dir: &Path,
        options: &TranspileOptions,
    ) -> Result<Vec<PathBuf>, NullScriptError> {
        let mut outputs = Vec::new();

        for entry in WalkDir::new(input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "ns"))
        {
            let ns_file = entry.path();
            let relative_path = ns_file.strip_prefix(input_dir)
                .map_err(|e| NullScriptError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)))?;

            let output_ext = match options.output_format {
                OutputFormat::JavaScript => "js",
                OutputFormat::TypeScript => "ts",
            };

            let output_path = output_dir.join(relative_path.with_extension(output_ext));

            match options.output_format {
                OutputFormat::JavaScript => {
                    self.transpile_to_js(ns_file, &output_path, options).await?;
                }
                OutputFormat::TypeScript => {
                    self.transpile_file(ns_file, &output_path, options).await?;
                }
            }

            outputs.push(output_path);
        }

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_basic_transpilation() {
        let transpiler = NullScriptTranspiler::new();
        let source = r#"
definitely message = "Hello, World!";
maybe count = 0;
checkthis (count is 0) {
    console.log(message);
}
"#;

        let result = transpiler.transpile(source, &TranspileOptions::default()).unwrap();

        assert!(result.contains("const message"));
        assert!(result.contains("let count"));
        assert!(result.contains("if (count === 0)"));
    }

    #[tokio::test]
    async fn test_function_transpilation() {
        let transpiler = NullScriptTranspiler::new();
        let source = r#"
feels greet(name: string): string {
    pls `Hello, ${name}!`;
}
"#;

        let result = transpiler.transpile(source, &TranspileOptions::default()).unwrap();

        assert!(result.contains("function greet"));
        assert!(result.contains("return `Hello"));
    }

    #[tokio::test]
    async fn test_syntax_validation() {
        let transpiler = NullScriptTranspiler::new();
        let source = "const invalid = 'should fail';";

        let result = transpiler.validate_syntax(source, None);
        assert!(result.is_err());
    }
}
