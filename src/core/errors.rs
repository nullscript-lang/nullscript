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
        let mut output = "❌ NullScriptTranspileError".to_string();
        output.push_str(&self.location.format());
        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

impl NullScriptSyntaxError {
    pub fn format_error(&self) -> String {
        let mut output = "❌ NullScriptSyntaxError".to_string();
        output.push_str(&self.location.format());
        output.push_str(&format!("\n\n{}", self.message));
        output
    }
}

impl NullScriptTypeError {
    pub fn format_error(&self) -> String {
        let mut output = "❌ NullScriptTypeError".to_string();
        output.push_str(&self.location.format());
        output.push_str(&format!("\n\n{}", self.message));
        output
    }
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
