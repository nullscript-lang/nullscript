use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NullScriptError {
    #[error("NullScriptTranspileError")]
    Transpile(#[from] NullScriptTranspileError),

    #[error("NullScriptSyntaxError")]
    Syntax(#[from] NullScriptSyntaxError),

    #[error("NullScriptTypeError")]
    Type(#[from] NullScriptTypeError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct NullScriptTranspileError {
    pub message: String,
    pub file_path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct NullScriptSyntaxError {
    pub message: String,
    pub file_path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct NullScriptTypeError {
    pub message: String,
    pub file_path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl NullScriptTranspileError {
    pub fn with_location(message: String, file_path: Option<PathBuf>, line: Option<u32>, column: Option<u32>) -> Self {
        Self {
            message,
            file_path,
            line,
            column,
        }
    }

    pub fn format_error(&self) -> String {
        let mut output = format!("❌ NullScriptTranspileError");

        if let Some(file_path) = &self.file_path {
            if let Some(file_name) = file_path.file_name() {
                output.push_str(&format!(" in {}", file_name.to_string_lossy()));
            }
        }

        if let Some(line) = self.line {
            output.push_str(&format!(":{}", line));
            if let Some(column) = self.column {
                output.push_str(&format!(":{}", column));
            }
        }

        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

impl NullScriptSyntaxError {
    pub fn with_location(message: String, file_path: Option<PathBuf>, line: Option<u32>, column: Option<u32>) -> Self {
        Self {
            message,
            file_path,
            line,
            column,
        }
    }

    pub fn format_error(&self) -> String {
        let mut output = format!("❌ NullScriptSyntaxError");

        if let Some(file_path) = &self.file_path {
            if let Some(file_name) = file_path.file_name() {
                output.push_str(&format!(" in {}", file_name.to_string_lossy()));
            }
        }

        if let Some(line) = self.line {
            output.push_str(&format!(":{}", line));
            if let Some(column) = self.column {
                output.push_str(&format!(":{}", column));
            }
        }

        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

impl NullScriptTypeError {
    pub fn format_error(&self) -> String {
        let mut output = format!("❌ NullScriptTypeError");

        if let Some(file_path) = &self.file_path {
            if let Some(file_name) = file_path.file_name() {
                output.push_str(&format!(" in {}", file_name.to_string_lossy()));
            }
        }

        if let Some(line) = self.line {
            output.push_str(&format!(":{}", line));
            if let Some(column) = self.column {
                output.push_str(&format!(":{}", column));
            }
        }

        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

pub struct ErrorMapping {
    pub message: String,
    pub suggestion: String,
}



pub fn get_error_mappings() -> std::collections::HashMap<&'static str, ErrorMapping> {
    let mut mappings = std::collections::HashMap::new();

    mappings.insert("Cannot find name 'feels'", ErrorMapping {
        message: "Invalid function declaration. Use 'feels' followed by a function name.".to_string(),
        suggestion: "Example: feels myFunction() { ... }".to_string(),
    });

    mappings.insert("Cannot find name 'definitely'", ErrorMapping {
        message: "Invalid variable declaration. Use 'definitely' for constants.".to_string(),
        suggestion: "Example: definitely myVar = 'value'".to_string(),
    });

    mappings.insert("Cannot find name 'maybe'", ErrorMapping {
        message: "Invalid variable declaration. Use 'maybe' for variables that can change.".to_string(),
        suggestion: "Example: maybe myVar = 'value'".to_string(),
    });

    mappings.insert("Cannot find name 'checkthis'", ErrorMapping {
        message: "Invalid conditional statement. Use 'checkthis' for if statements.".to_string(),
        suggestion: "Example: checkthis (condition) { ... }".to_string(),
    });

    mappings.insert("Cannot find name 'orelse'", ErrorMapping {
        message: "Invalid else statement. Use 'orelse' for else clauses.".to_string(),
        suggestion: "Example: checkthis (condition) { ... } orelse { ... }".to_string(),
    });

    mappings.insert("Cannot find name 'pls'", ErrorMapping {
        message: "Invalid return statement. Use 'pls' to return values.".to_string(),
        suggestion: "Example: pls myValue".to_string(),
    });

    mappings.insert("Cannot find name 'fr'", ErrorMapping {
        message: "Invalid boolean value. Use 'fr' for true.".to_string(),
        suggestion: "Example: definitely isValid = fr".to_string(),
    });

    mappings.insert("Cannot find name 'cap'", ErrorMapping {
        message: "Invalid boolean value. Use 'cap' for false.".to_string(),
        suggestion: "Example: definitely isValid = cap".to_string(),
    });

    mappings.insert("Cannot find name 'nocap'", ErrorMapping {
        message: "Invalid null value. Use 'nocap' for null.".to_string(),
        suggestion: "Example: definitely value = nocap".to_string(),
    });

    mappings.insert("Cannot find name 'ghost'", ErrorMapping {
        message: "Invalid undefined value. Use 'ghost' for undefined.".to_string(),
        suggestion: "Example: definitely value = ghost".to_string(),
    });

    mappings.insert("Cannot find name 'vibes'", ErrorMapping {
        message: "Invalid interface declaration. Use 'vibes' to define interfaces.".to_string(),
        suggestion: "Example: vibes MyInterface { ... }".to_string(),
    });

    mappings.insert("Cannot find name 'vibe'", ErrorMapping {
        message: "Invalid type alias. Use 'vibe' to define type aliases.".to_string(),
        suggestion: "Example: vibe MyType = string | number".to_string(),
    });

    mappings.insert("Cannot find name 'bigbrain'", ErrorMapping {
        message: "Invalid class declaration. Use 'bigbrain' to define classes.".to_string(),
        suggestion: "Example: bigbrain MyClass { ... }".to_string(),
    });

    mappings.insert("Unexpected token", ErrorMapping {
        message: "Syntax error in NullScript code. Check for missing keywords or incorrect syntax.".to_string(),
                    suggestion: "Make sure you're using NullScript keywords correctly. Run 'nsc keywords' to see all available keywords.".to_string(),
    });

    mappings.insert("Declaration or statement expected", ErrorMapping {
        message: "Invalid statement. Check your NullScript syntax.".to_string(),
        suggestion: "Make sure you're using proper NullScript keywords and syntax.".to_string(),
    });

    mappings.insert("Function implementation is missing", ErrorMapping {
        message: "Function body is missing. Add implementation after your function declaration.".to_string(),
        suggestion: "Example: feels myFunction() { /* your code here */ }".to_string(),
    });

    mappings.insert("Unexpected keyword or identifier", ErrorMapping {
        message: "Invalid NullScript syntax. You're using an undefined keyword or incorrect syntax.".to_string(),
                    suggestion: "Check that you're using valid NullScript keywords. Run 'nsc keywords' to see all available options.".to_string(),
    });

    mappings
}

pub fn parse_typescript_error(error_output: &str, file_path: Option<PathBuf>) -> NullScriptError {
    let lines: Vec<&str> = error_output.split('\n').collect();
    let mut line: Option<u32> = None;
    let mut column: Option<u32> = None;
    let mut error_message = error_output.to_string();

    if let Some(location_match) = regex::Regex::new(r"(\w+\.ts):(\d+):(\d+)\s*-\s*error|:(\d+):(\d+)")
        .ok()
        .and_then(|re| re.captures(error_output))
    {
        if let Some(line_str) = location_match.get(2).or_else(|| location_match.get(4)) {
            line = line_str.as_str().parse().ok();
        }
        if let Some(col_str) = location_match.get(3).or_else(|| location_match.get(5)) {
            column = col_str.as_str().parse().ok();
        }
    }

    let error_lines: Vec<&str> = lines.iter().filter(|line| line.contains("error TS")).cloned().collect();
    if !error_lines.is_empty() {
        let first_error = error_lines[0];
        if let Some(captures) = regex::Regex::new(r"error TS\d+: (.+)")
            .ok()
            .and_then(|re| re.captures(first_error))
        {
            if let Some(matched) = captures.get(1) {
                error_message = matched.as_str().to_string();
            }
        }
    }

    if error_lines.is_empty() {
        if let Some(compilation_error) = lines.iter().find(|line| {
            line.contains("Cannot find name") ||
            line.contains("Unexpected token") ||
            line.contains("Declaration or statement expected")
        }) {
            error_message = compilation_error.trim().to_string();
        }
    }

    let error_mappings = get_error_mappings();
    for (pattern, mapping) in error_mappings.iter() {
        if error_message.contains(pattern) {
            let custom_message = format!("{}\n💡 {}", mapping.message, mapping.suggestion);

            return NullScriptError::Syntax(
                NullScriptSyntaxError::with_location(custom_message, file_path, line, column)
            );
        }
    }

    let clean_message = error_message
        .replace(regex::Regex::new(r"error TS\d+:\s*").unwrap().as_str(), "")
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .filter(|line| !line.starts_with("at "))
        .filter(|line| !line.contains("Command failed:"))
        .filter(|line| !line.contains("(node:"))
        .take(3)
        .collect::<Vec<&str>>()
        .join("\n")
        .trim()
        .to_string();

    let fallback_message = format!(
        "Transpilation error: {}\n💡 This might be due to incorrect NullScript syntax. Run 'nsc keywords' to see available keywords.",
        clean_message
    );

    NullScriptError::Transpile(
        NullScriptTranspileError::with_location(fallback_message, file_path, line, column)
    )
}

pub fn format_error(error: &NullScriptError) -> String {
    match error {
        NullScriptError::Transpile(e) => e.format_error(),
        NullScriptError::Syntax(e) => e.format_error(),
        NullScriptError::Type(e) => e.format_error(),
        NullScriptError::Io(e) => format!("❌ IO Error: {}", e),
        NullScriptError::Regex(e) => format!("❌ Regex Error: {}", e),
        NullScriptError::Json(e) => format!("❌ JSON Error: {}", e),
    }
}
