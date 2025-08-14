
use crate::errors::types::{NullScriptError, NullScriptSyntaxError};
use crate::language::keywords::{KEYWORDS, FORBIDDEN_KEYWORDS, INVALID_SYNTAX};
use crate::language::types::{Location, WithLocation};
use regex::Regex;
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;

pub struct NullScriptTranspiler {}

impl Default for NullScriptTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

impl NullScriptTranspiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate_syntax(&self, source: &str, file_path: Option<&Path>) -> Result<(), NullScriptError> {
        let file_name = file_path.map(|p| p.to_string_lossy()).unwrap_or_else(|| "unknown".into());


        let lines: Vec<&str> = source.split('\n').collect();
        let mut code_without_comments = String::new();

        for line in lines {
            let trimmed_line = line.trim();


            if trimmed_line.is_empty() || trimmed_line.starts_with("//") || trimmed_line.starts_with("/*") {
                continue;
            }


            if let Some(comment_start) = line.find("//") {
                code_without_comments.push_str(&line[..comment_start]);
            } else {
                code_without_comments.push_str(line);
            }
            code_without_comments.push('\n');
        }


        for keyword in FORBIDDEN_KEYWORDS.iter() {
            let pattern = format!(r"\b{}\b", regex::escape(keyword));
            if let Ok(regex) = Regex::new(&pattern) {
                if regex.is_match(&code_without_comments) {
                    let message = format!(
                        "Forbidden TypeScript keyword '{}' found in NullScript file '{}'.\n❌ TypeScript syntax is not allowed in NullScript files.",
                        keyword, file_name
                    );
                    let location = Location::new(
                        file_path.map(|p| p.to_path_buf()),
                        Some(1),
                        None,
                    );
                    return Err(NullScriptError::Syntax(
                        NullScriptSyntaxError::with_location(message, location)
                    ));
                }
            }
        }

        for pattern in INVALID_SYNTAX.iter() {

            if pattern.contains(' ') || pattern.contains(':') || pattern.contains('<') || pattern.contains('>') {
                if code_without_comments.contains(pattern) {
                    let message = format!(
                        "Invalid TypeScript syntax '{}' found in NullScript file '{}'.\n❌ TypeScript syntax is not allowed in NullScript files.",
                        pattern, file_name
                    );
                    let location = Location::new(
                        file_path.map(|p| p.to_path_buf()),
                        Some(1),
                        None,
                    );
                    return Err(NullScriptError::Syntax(
                        NullScriptSyntaxError::with_location(message, location)
                    ));
                }
            } else {

                let word_pattern = format!(r"\b{}\b", regex::escape(pattern));
                if let Ok(regex) = Regex::new(&word_pattern) {
                    if regex.is_match(&code_without_comments) {
                        let message = format!(
                            "Invalid TypeScript syntax '{}' found in NullScript file '{}'.\n❌ TypeScript syntax is not allowed in NullScript files.",
                            pattern, file_name
                        );
                        let location = Location::new(
                            file_path.map(|p| p.to_path_buf()),
                            Some(1),
                            None,
                        );
                        return Err(NullScriptError::Syntax(
                            NullScriptSyntaxError::with_location(message, location)
                        ));
                    }
                }
            }
        }


        let type_annotation_patterns = [
            r":\s*[A-Za-z_$][\w$<>|[\]\s]*\s*[=,)]",
            r"\)\s*:\s*[A-Za-z_$][\w$<>|[\]\s]*\s*\{",
            r"run\s+[a-zA-Z_$][\w$]*\s*\([^)]*\)\s*:\s*[A-Za-z_$][\w$<>|[\]\s]*",
        ];

        for pattern in type_annotation_patterns.iter() {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(source) {
                    let message = format!(
                        "TypeScript type annotations found in NullScript file '{}'.\n❌ TypeScript syntax is not allowed in NullScript files.",
                        file_name
                    );
                    let location = Location::new(
                        file_path.map(|p| p.to_path_buf()),
                        Some(1),
                        None,
                    );
                    return Err(NullScriptError::Syntax(
                        NullScriptSyntaxError::with_location(message, location)
                    ));
                }
            }
        }


        let lines: Vec<&str> = source.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            let line = line.trim();
            let line_number = i as u32 + 1;

            if line.is_empty() || line.starts_with("//") || line.starts_with("/*") {
                continue;
            }


            let invalid_patterns = vec![
                (r"^\s*(function\s+\w+\s*\()", "using 'function' instead of 'run'"),
                (r"^\s*(const\s+\w+)", "using 'const' instead of 'fixed'"),
                (r"^\s*(if\s*\()", "using 'if' instead of 'whatever'"),
                (r"^\s*(else\s+)", "using 'else' instead of 'otherwise'"),
                (r"^\s*(true)\b", "using 'true' instead of 'yes'"),
                (r"^\s*(false)\b", "using 'false' instead of 'no'"),
                (r"^\s*(class\s+\w+)", "using 'class' instead of 'model'"),
                (r"^\s*(try\s*\{)", "using 'try' instead of 'test'"),
                (r"^\s*(catch\s*\()", "using 'catch' instead of 'grab'"),
                (r"^\s*(finally\s*\{)", "using 'finally' instead of 'atLast'"),
            ];

            for (pattern, description) in invalid_patterns {
                let regex = Regex::new(pattern)?;
                if regex.is_match(line) {
                    let message = format!(
                        "Invalid syntax on line {}: {}\n💡 Use NullScript keywords instead of standard JavaScript/TypeScript syntax.",
                        line_number, description
                    );
                    let location = Location::new(
                        file_path.map(|p| p.to_path_buf()),
                        Some(line_number),
                        None,
                    );
                    return Err(NullScriptError::Syntax(
                        NullScriptSyntaxError::with_location(message, location)
                    ));
                }
            }
        }


        let nullscript_keywords: Vec<&str> = KEYWORDS.iter().map(|(keyword, _)| *keyword).collect();


        let identifier_patterns = vec![
            (r"^\s*(let|fixed|var)\s+([a-zA-Z_$][\w$]*)\s*=", "variable declaration", 2),
            (r"^\s*run\s+([a-zA-Z_$][\w$]*)\s*\(", "function declaration", 1),
            (r"^\s*model\s+([a-zA-Z_$][\w$]*)\s*\{", "class declaration", 1),
            (r"^\s+run\s+([a-zA-Z_$][\w$]*)\s*\(", "method declaration", 1),
        ];

        for (pattern, description, capture_group) in identifier_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(source) {
                    if let Some(identifier) = cap.get(capture_group) {
                        let clean_id = identifier.as_str().trim();
                        if nullscript_keywords.contains(&clean_id) {
                            let message = format!(
                                "Cannot use NullScript keyword '{}' as {}.\n💡 Choose a different name for your {}.",
                                clean_id, description, description
                            );
                            let location = Location::new(
                                file_path.map(|p| p.to_path_buf()),
                                Some(1),
                                None,
                            );
                            return Err(NullScriptError::Syntax(
                                NullScriptSyntaxError::with_location(message, location)
                            ));
                        }
                    }
                }
            }
        }


        let param_pattern = Regex::new(r"run\s+[a-zA-Z_$][\w$]*\s*\(([^)]*)\)")?;
        for cap in param_pattern.captures_iter(source) {
            if let Some(params_str) = cap.get(1) {
                let params = params_str.as_str().split(',').map(|p| p.trim()).collect::<Vec<_>>();
                for param in params {
                    if !param.is_empty() && nullscript_keywords.contains(&param) {
                        let message = format!(
                            "Cannot use NullScript keyword '{}' as function parameter.\n💡 Choose a different name for your function parameter.",
                            param
                        );
                        let location = Location::new(
                            file_path.map(|p| p.to_path_buf()),
                            Some(1),
                            None,
                        );
                        return Err(NullScriptError::Syntax(
                            NullScriptSyntaxError::with_location(message, location)
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn transpile(&self, source: &str) -> Result<String, NullScriptError> {
        let mut output = source.to_string();


        let class_decl_regex = Regex::new(r"model\s+([a-zA-Z_$][\w$]*)\s*\{")?;
        output = class_decl_regex.replace_all(&output, "class $1 {").to_string();


        let class_field_decl_pattern = Regex::new(r"(\s{4,})fixed\s+([a-zA-Z_$][\w$]*)\s*;")?;
        output = class_field_decl_pattern.replace_all(&output, "").to_string();


        let class_field_pattern = Regex::new(r"(\s{4,})fixed\s+([a-zA-Z_$][\w$]*)\s*=\s*([^;]+);")?;
        output = class_field_pattern.replace_all(&output, "").to_string();




        let static_regex = Regex::new(r"\brun\s+forever\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = static_regex.replace_all(&output, "static $1($2) {").to_string();


        let async_top_regex = Regex::new(r"(?m)^\s*run\s+later\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = async_top_regex.replace_all(&output, "async function $1($2) {").to_string();


        let function_declaration_regex = Regex::new(r"run\s+([a-zA-Z_$][\w$]*)\s*\(\s*\)\s*\{")?;
        output = function_declaration_regex.replace_all(&output, "function $1() {").to_string();


        let function_declaration_params_regex = Regex::new(r"run\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = function_declaration_params_regex.replace_all(&output, "function $1($2) {").to_string();


        let nested_function_regex = Regex::new(r"(\s*)run\s+([a-zA-Z_$][\w$]*)\s*\(\s*\)\s*\{")?;
        output = nested_function_regex.replace_all(&output, "$1function $2() {").to_string();


        let nested_function_params_regex = Regex::new(r"(\s*)run\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = nested_function_params_regex.replace_all(&output, "$1function $2($3) {").to_string();


        let class_method_post_regex = Regex::new(r"(\s{4,})function\s+([a-zA-Z_$][\w$]*)\s*\(\s*\)\s*\{")?;
        output = class_method_post_regex.replace_all(&output, "$1$2() {").to_string();


        let class_method_params_post_regex = Regex::new(r"(\s{4,})function\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = class_method_params_post_regex.replace_all(&output, "$1$2($3) {").to_string();


        let constructor_regex = Regex::new(r"(\s{4,})function\s+__init__\s*\(([^)]*)\)\s*\{")?;
        output = constructor_regex.replace_all(&output, "$1constructor($2) {").to_string();


        let constructor_run_regex = Regex::new(r"(\s{4,})run\s+__init__\s*\(([^)]*)\)\s*\{")?;
        output = constructor_run_regex.replace_all(&output, "$1constructor($2) {").to_string();


        let async_method_regex = Regex::new(r"(\s{4,})async\s+function\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = async_method_regex.replace_all(&output, "$1async $2($3) {").to_string();


        let async_method_fix_regex = Regex::new(r"(\s{4,})function\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{(\s*await)")?;
        output = async_method_fix_regex.replace_all(&output, "$1async $2($3) {$4").to_string();


        let class_async_regex = Regex::new(r"(\s{4,})function\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{(\s*let\s+response\s*=\s*await)")?;
        output = class_async_regex.replace_all(&output, "$1async $2($3) {$4").to_string();


        let standalone_async_regex = Regex::new(r"(?m)\brun\s+async\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = standalone_async_regex.replace_all(&output, "async function $1($2) {").to_string();


        let class_run_async_regex = Regex::new(r"(?m)(\s{4,})run\s+async\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = class_run_async_regex.replace_all(&output, "$1async $2($3) {").to_string();




        let remove_regex = Regex::new(r"\bremove\s+([a-zA-Z_$][\w$]*(?:\.[a-zA-Z_$][\w$]*)*(?:\[[^\]]+\])?)\b")?;
        output = remove_regex.replace_all(&output, "delete $1").to_string();

        for (nullscript_keyword, js_keyword) in KEYWORDS.iter() {

            if *nullscript_keyword == "run" || *nullscript_keyword == "remove" {
                continue;
            }

            let pattern = format!(r"\b{}\b", regex::escape(nullscript_keyword));
            let regex = Regex::new(&pattern)?;
            output = regex.replace_all(&output, *js_keyword).to_string();
        }


        let default_export_regex = Regex::new(r"\bshare\s+default\s+run\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = default_export_regex.replace_all(&output, "export default function $1($2) {").to_string();


        let object_function_regex = Regex::new(r"(\w+)\s*:\s*run\s*\(")?;
        output = object_function_regex.replace_all(&output, "$1: function(").to_string();


        let arrow_function_regex = Regex::new(r"run\s*\(([^)]*)\)\s*\{")?;
        output = arrow_function_regex.replace_all(&output, "function($1) {").to_string();


        let non_null_regex = Regex::new(r"([a-zA-Z_$][\w$]*)\!")?;
        output = non_null_regex.replace_all(&output, "$1").to_string();


        let super_constructor_regex = Regex::new(r"super\.constructor\(")?;
        output = super_constructor_regex.replace_all(&output, "super(").to_string();


        let json_method_regex = Regex::new(r"\.JSON\(")?;
        output = json_method_regex.replace_all(&output, ".json(").to_string();


        let static_method_call_regex = Regex::new(r"([a-zA-Z_$][\w$]*)\.forever\.([a-zA-Z_$][\w$]*)\(")?;
        output = static_method_call_regex.replace_all(&output, "$1.$2(").to_string();


        let static_call_regex = Regex::new(r"([a-zA-Z_$][\w$]*)\.static\.([a-zA-Z_$][\w$]*)\(")?;
        output = static_call_regex.replace_all(&output, "$1.$2(").to_string();


        let default_import_regex = Regex::new(r"\bimport\s+default\s+as\s+([a-zA-Z_$][\w$]*)")?;
        output = default_import_regex.replace_all(&output, "import $1").to_string();





        Ok(output)
    }

    pub async fn transpile_to_js(
        &self,
        ns_path: &Path,
        js_path: &Path,
    ) -> Result<(), NullScriptError> {
        let source = fs::read_to_string(ns_path).await?;

        self.validate_syntax(&source, Some(ns_path))?;

        let transpiled = self.transpile(&source)?;

        if let Some(parent) = js_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(js_path, &transpiled).await?;

        Ok(())
    }

    pub async fn build_directory(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, NullScriptError> {
        let mut outputs = Vec::new();

        for entry in WalkDir::new(input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "ns"))
        {
            let ns_file = entry.path();
            let relative_path = ns_file.strip_prefix(input_dir)
                .map_err(|e| NullScriptError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)))?;

            let output_path = output_dir.join(relative_path.with_extension("js"));

            self.transpile_to_js(ns_file, &output_path).await?;

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
fixed message = "Hello, World!";
let count = 0;
whatever (count is 0) {
    console.log(message);
}
"#;

        let result = transpiler.transpile(source).unwrap();

        assert!(result.contains("const message"));
        assert!(result.contains("let count"));
        assert!(result.contains("if (count === 0)"));
    }

    #[tokio::test]
    async fn test_function_transpilation() {
        let transpiler = NullScriptTranspiler::new();
        let source = r#"
run greet(name: string): string {
    result `Hello, ${name}!`;
}
"#;

        let result = transpiler.transpile(source).unwrap();

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
