use crate::errors::types::NullScriptError;
use colored::Colorize;

/// Error formatting utilities
pub struct ErrorFormatter;

impl ErrorFormatter {
    /// Formats an error for user display
    pub fn format_error(error: &NullScriptError) -> String {
        match error {
            NullScriptError::Io(io_error) => {
                format!("❌ I/O Error: {}", io_error.to_string().red())
            }
            NullScriptError::Json(json_error) => {
                format!("❌ JSON Error: {}", json_error.to_string().red())
            }
            NullScriptError::Transpile(transpile_error) => {
                format!("❌ Transpilation Error: {}", transpile_error.message.red())
            }
            NullScriptError::Syntax(syntax_error) => {
                format!("❌ Syntax Error: {}", syntax_error.message.red())
            }
            NullScriptError::Type(type_error) => {
                format!("❌ Type Error: {}", type_error.message.red())
            }
            NullScriptError::Regex(regex_error) => {
                format!("❌ Regex Error: {}", regex_error.to_string().red())
            }
        }
    }


}
