use serde::{Deserialize, Serialize};

/// Diagnostic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Diagnostic message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub range: TextRange,
    pub code: Option<String>,
    pub source: Option<String>,
}

/// Text range for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRange {
    pub start: Position,
    pub end: Position,
}

/// Position in text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}


