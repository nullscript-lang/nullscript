use crate::config::schema::NullScriptConfigSchema;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};

// Re-export the schema as the main config type
pub type NullScriptConfig = NullScriptConfigSchema;



impl NullScriptConfig {
    /// Loads configuration from nsconfig.json with strict schema validation
    /// This will throw compilation errors if the schema doesn't match exactly
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        // Use strict schema validation
        Self::validate_json(&content)
            .with_context(|| format!("Schema validation failed for: {}", path.display()))
    }

    /// Loads configuration from nsconfig.json or returns default if not found
    /// If nsconfig.json exists but is invalid, this will cause compilation to fail
    pub fn load_or_default(project_root: &Path) -> Self {
        let config_path = project_root.join("nsconfig.json");

        if config_path.exists() {
            match Self::load_from_file(&config_path) {
                Ok(config) => {
                    println!("✅ Loaded nsconfig.json successfully");
                    config
                },
                Err(e) => {
                    // This is intentionally fatal - we don't want to silently fall back
                    // to defaults if there's a schema violation
                    eprintln!("❌ FATAL ERROR: Invalid nsconfig.json");
                    eprintln!("   {}", e);
                    eprintln!("   Fix the configuration file or remove it to use defaults.");
                    eprintln!("   Run 'nsc config --generate' to create a valid nsconfig.json");
                    std::process::exit(1);
                }
            }
        } else {
            // No config file found, use default values
            println!("ℹ️  No nsconfig.json found, using default configuration");
            Self::default()
        }
    }

    /// Saves configuration to nsconfig.json with proper formatting
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = self.to_pretty_json()
            .context("Failed to serialize configuration")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Creates a default nsconfig.json file
    pub fn create_default_config(project_root: &Path) -> Result<()> {
        let config_path = project_root.join("nsconfig.json");
        let default_config = Self::default();
        default_config.save_to_file(&config_path)?;
        Ok(())
    }

    /// Validates an existing nsconfig.json file without loading it
    pub fn validate_file(path: &PathBuf) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        Self::validate_json(&content)
            .with_context(|| format!("Schema validation failed for: {}", path.display()))?;

        Ok(())
    }

    // Helper methods to maintain compatibility with existing code

    /// Gets exclude patterns (similar to old development.ignore_patterns)
    pub fn get_exclude_patterns(&self) -> Vec<String> {
        self.exclude.clone()
    }
}
