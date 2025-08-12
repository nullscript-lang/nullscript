use std::path::PathBuf;
use thiserror::Error;
use crate::core::types::{Location, WithLocation};

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
    pub location: Location,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct NullScriptSyntaxError {
    pub message: String,
    pub location: Location,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct NullScriptTypeError {
    pub message: String,
    pub location: Location,
}

impl WithLocation for NullScriptTranspileError {
    fn with_location(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

impl WithLocation for NullScriptSyntaxError {
    fn with_location(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

impl WithLocation for NullScriptTypeError {
    fn with_location(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

impl NullScriptTranspileError {
    pub fn format_error(&self) -> String {
        let mut output = format!("❌ NullScriptTranspileError");
        output.push_str(&self.location.format());
        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

impl NullScriptSyntaxError {
    pub fn format_error(&self) -> String {
        let mut output = format!("❌ NullScriptSyntaxError");
        output.push_str(&self.location.format());
        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

impl NullScriptTypeError {
    pub fn format_error(&self) -> String {
        let mut output = format!("❌ NullScriptTypeError");
        output.push_str(&self.location.format());
        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

#[derive(Debug)]
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
    use crate::utils::regex::RegexUtils;
    use crate::utils::strings::StringUtils;

    let lines: Vec<&str> = error_output.split('\n').collect();
    let (line, column) = RegexUtils::extract_location(error_output);
    let mut error_message = error_output.to_string();

    if let Some(ts_error) = RegexUtils::extract_ts_error(error_output) {
        error_message = ts_error;
    } else {
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
            let location = Location::new(file_path, line, column);
            return NullScriptError::Syntax(
                NullScriptSyntaxError::with_location(custom_message, location)
            );
        }
    }

    let clean_message = StringUtils::clean_error_message(&error_message);
    let fallback_message = format!(
        "Transpilation error: {}\n💡 This might be due to incorrect NullScript syntax. Run 'nsc keywords' to see available keywords.",
        clean_message
    );

    let location = Location::new(file_path, line, column);
    NullScriptError::Transpile(
        NullScriptTranspileError::with_location(fallback_message, location)
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
