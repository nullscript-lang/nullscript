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


}

pub trait WithLocation {
    fn with_location(message: String, location: Location) -> Self;
}
