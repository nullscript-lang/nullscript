use thiserror::Error;
use crate::language::types::{Location, WithLocation};

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


