use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file_path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl Location {
    pub fn new(file_path: Option<PathBuf>, line: Option<u32>, column: Option<u32>) -> Self {
        Self {
            file_path,
            line,
            column,
        }
    }

    pub fn format(&self) -> String {
        let mut output = String::new();

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

        output
    }
}

pub trait WithLocation {
    fn with_location(message: String, location: Location) -> Self;
}
