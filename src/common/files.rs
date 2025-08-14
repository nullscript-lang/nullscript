use std::path::{Path, PathBuf};
use std::fs;
use crate::errors::types::NullScriptError;

pub struct FileUtils;

impl FileUtils {
    pub fn get_extension(file_path: &Path) -> Option<String> {
        file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    pub fn get_stem(file_path: &Path) -> Option<String> {
        file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }

    pub fn has_extension(file_path: &Path, extension: &str) -> bool {
        Self::get_extension(file_path)
            .map(|ext| ext == extension)
            .unwrap_or(false)
    }

    pub fn get_file_size(file_path: &PathBuf) -> Result<u64, NullScriptError> {
        let metadata = fs::metadata(file_path)?;
        Ok(metadata.len())
    }

    pub fn format_file_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        match size {
            0..KB => format!("{} B", size),
            KB..MB => format!("{:.1} KB", size as f64 / KB as f64),
            MB..GB => format!("{:.1} MB", size as f64 / MB as f64),
            _ => format!("{:.1} GB", size as f64 / GB as f64),
        }
    }

    pub fn is_nullscript_file(file_path: &Path) -> bool {
        Self::has_extension(file_path, "ns")
    }

    pub fn count_lines(file_path: &PathBuf) -> Result<usize, NullScriptError> {
        let content = fs::read_to_string(file_path)?;
        Ok(content.lines().count())
    }

    pub fn get_modified_time(file_path: &PathBuf) -> Result<std::time::SystemTime, NullScriptError> {
        let metadata = fs::metadata(file_path)?;
        Ok(metadata.modified()?)
    }
}
