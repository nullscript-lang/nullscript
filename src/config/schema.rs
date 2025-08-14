use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use anyhow::{Result, anyhow, Context};

/// Strict schema definition for nsconfig.json
/// Any deviation from this schema will result in compilation error
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NullScriptConfigSchema {
    #[serde(rename = "compilerOptions")]
    pub compiler_options: CompilerOptions,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompilerOptions {
    #[serde(rename = "rootDir")]
    pub root_dir: String,
    #[serde(rename = "outDir")]
    pub out_dir: String,
    pub reports: ReportsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportsConfig {
    pub dir: String,
    #[serde(rename = "defaultFormat")]
    pub default_format: String,
}

impl Default for NullScriptConfigSchema {
    fn default() -> Self {
        Self {
            compiler_options: CompilerOptions::default(),
            include: vec!["src/**/*.ns".to_string()],
            exclude: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                "reports".to_string(),
            ],
        }
    }
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            root_dir: "./src".to_string(),
            out_dir: "./dist".to_string(),
            reports: ReportsConfig::default(),
        }
    }
}

impl Default for ReportsConfig {
    fn default() -> Self {
        Self {
            dir: "reports".to_string(),
            default_format: "html".to_string(),
        }
    }
}

impl NullScriptConfigSchema {
    /// Validates JSON content against the strict schema
    /// Returns error if:
    /// 1. JSON structure doesn't match exactly
    /// 2. Extra fields are present
    /// 3. Required fields are missing
    /// 4. Field types don't match
    pub fn validate_json(json_content: &str) -> Result<Self> {
        // First, parse as generic JSON to check for extra fields
        let json_value: Value = serde_json::from_str(json_content)
            .context("Invalid JSON format in nsconfig.json")?;

        // Validate that only expected fields are present
        Self::validate_no_extra_fields(&json_value)?;

        // Parse into our strict schema
        let config: NullScriptConfigSchema = serde_json::from_str(json_content)
            .context("nsconfig.json structure doesn't match required schema")?;

        // Additional validation
        Self::validate_field_values(&config)?;

        Ok(config)
    }

    /// Validates that no extra fields are present beyond the schema
    fn validate_no_extra_fields(value: &Value) -> Result<()> {
        if let Value::Object(obj) = value {
            let allowed_root_fields: HashSet<&str> =
                ["compilerOptions", "include", "exclude"].iter().cloned().collect();

            for key in obj.keys() {
                if !allowed_root_fields.contains(key.as_str()) {
                    return Err(anyhow!(
                        "Unknown field '{}' in nsconfig.json. Only 'compilerOptions', 'include', and 'exclude' are allowed.",
                        key
                    ));
                }
            }

            // Validate compilerOptions sub-fields
            if let Some(Value::Object(compiler_opts)) = obj.get("compilerOptions") {
                let allowed_compiler_fields: HashSet<&str> =
                    ["rootDir", "outDir", "reports"].iter().cloned().collect();

                for key in compiler_opts.keys() {
                    if !allowed_compiler_fields.contains(key.as_str()) {
                        return Err(anyhow!(
                            "Unknown field '{}' in compilerOptions. Only 'rootDir', 'outDir', and 'reports' are allowed.",
                            key
                        ));
                    }
                }

                // Validate reports sub-fields
                if let Some(Value::Object(reports)) = compiler_opts.get("reports") {
                    let allowed_reports_fields: HashSet<&str> =
                        ["dir", "defaultFormat"].iter().cloned().collect();

                    for key in reports.keys() {
                        if !allowed_reports_fields.contains(key.as_str()) {
                            return Err(anyhow!(
                                "Unknown field '{}' in reports. Only 'dir' and 'defaultFormat' are allowed.",
                                key
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validates field values are reasonable
    fn validate_field_values(config: &NullScriptConfigSchema) -> Result<()> {
        // Validate root_dir and out_dir are not empty
        if config.compiler_options.root_dir.trim().is_empty() {
            return Err(anyhow!("compilerOptions.rootDir cannot be empty"));
        }

        if config.compiler_options.out_dir.trim().is_empty() {
            return Err(anyhow!("compilerOptions.outDir cannot be empty"));
        }

        // Validate reports dir is not empty
        if config.compiler_options.reports.dir.trim().is_empty() {
            return Err(anyhow!("compilerOptions.reports.dir cannot be empty"));
        }

        // Validate reports format is supported
        let supported_formats = ["html", "json", "text"];
        if !supported_formats.contains(&config.compiler_options.reports.default_format.as_str()) {
            return Err(anyhow!(
                "compilerOptions.reports.defaultFormat must be one of: {}. Got: '{}'",
                supported_formats.join(", "),
                config.compiler_options.reports.default_format
            ));
        }

        // Validate include patterns are not empty
        if config.include.is_empty() {
            return Err(anyhow!("include array cannot be empty"));
        }

        for pattern in &config.include {
            if pattern.trim().is_empty() {
                return Err(anyhow!("include patterns cannot be empty strings"));
            }
        }

        // Validate exclude patterns (allow empty, but no empty strings)
        for pattern in &config.exclude {
            if pattern.trim().is_empty() {
                return Err(anyhow!("exclude patterns cannot be empty strings"));
            }
        }

        Ok(())
    }

    /// Creates a properly formatted JSON string for nsconfig.json
    pub fn to_pretty_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .context("Failed to serialize config to JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let json = r#"{
            "compilerOptions": {
                "rootDir": "./src",
                "outDir": "./dist",
                "reports": {
                    "dir": "reports",
                    "defaultFormat": "html"
                }
            },
            "include": ["src/**/*.ns"],
            "exclude": ["node_modules", "dist", "reports"]
        }"#;

        let result = NullScriptConfigSchema::validate_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extra_field_rejection() {
        let json = r#"{
            "compilerOptions": {
                "rootDir": "./src",
                "outDir": "./dist",
                "reports": {
                    "dir": "reports",
                    "defaultFormat": "html"
                }
            },
            "include": ["src/**/*.ns"],
            "exclude": ["node_modules", "dist", "reports"],
            "extraField": "should fail"
        }"#;

        let result = NullScriptConfigSchema::validate_json(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown field 'extraField'"));
    }

    #[test]
    fn test_invalid_format_rejection() {
        let json = r#"{
            "compilerOptions": {
                "rootDir": "./src",
                "outDir": "./dist",
                "reports": {
                    "dir": "reports",
                    "defaultFormat": "invalid"
                }
            },
            "include": ["src/**/*.ns"],
            "exclude": ["node_modules", "dist", "reports"]
        }"#;

        let result = NullScriptConfigSchema::validate_json(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("defaultFormat must be one of"));
    }

    #[test]
    fn test_default_config() {
        let default_config = NullScriptConfigSchema::default();
        let json = default_config.to_pretty_json().unwrap();
        let parsed = NullScriptConfigSchema::validate_json(&json).unwrap();
        assert_eq!(default_config, parsed);
    }
}
