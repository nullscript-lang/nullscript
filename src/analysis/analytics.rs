use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub build_id: String,
    pub timestamp: u64,
    pub total_build_time_ms: u64,
    pub file_metrics: Vec<FileMetrics>,
    pub bundle_analysis: BundleAnalysis,
    pub memory_usage: Option<MemoryUsage>,
    pub system_info: SystemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub file_path: PathBuf,
    pub input_size_bytes: u64,
    pub output_size_bytes: u64,
    pub transpile_time_ms: u64,
    pub line_count: u32,
    pub character_count: u32,
    pub complexity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleAnalysis {
    pub total_input_size: u64,
    pub total_output_size: u64,
    pub compression_ratio: f64,
    pub file_count: u32,
    pub dependency_graph: DependencyGraph,
    pub duplicate_detection: Vec<DuplicateFile>,
    pub largest_files: Vec<FileSize>,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: Vec<DependencyNode>,
    pub edges: Vec<DependencyEdge>,
    pub circular_dependencies: Vec<CircularDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: String,
    pub file_path: PathBuf,
    pub size_bytes: u64,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub import_type: ImportType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportType {
    Named,
    Default,
    Namespace,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularDependency {
    pub cycle: Vec<String>,
    pub severity: CycleSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CycleSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFile {
    pub content_hash: String,
    pub files: Vec<PathBuf>,
    pub size_bytes: u64,
    pub similarity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSize {
    pub file_path: PathBuf,
    pub size_bytes: u64,
    pub percentage_of_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub suggestion_type: OptimizationType,
    pub description: String,
    pub potential_savings_bytes: u64,
    pub priority: OptimizationPriority,
    pub files_affected: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    RemoveDuplicates,
    TreeShaking,
    CodeSplitting,
    ConstantFolding,
    DeadCodeElimination,
    FunctionInlining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub peak_memory_mb: f64,
    pub average_memory_mb: f64,
    pub garbage_collections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub cpu_cores: u32,
    pub total_memory_mb: u64,
    pub node_version: Option<String>,
    pub rust_version: String,
}

pub struct PerformanceAnalyzer {
    start_time: Option<Instant>,
    file_start_times: HashMap<PathBuf, Instant>,
    metrics: Vec<FileMetrics>,
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            file_start_times: HashMap::new(),
            metrics: Vec::new(),
        }
    }

    pub fn start_build(&mut self) {
        self.start_time = Some(Instant::now());
        self.metrics.clear();
        println!("📊 Performance analysis started");
    }

    pub fn start_file(&mut self, file_path: PathBuf) {
        self.file_start_times.insert(file_path, Instant::now());
    }

    pub fn finish_file(&mut self, file_path: PathBuf, input_content: &str, output_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let end_time = Instant::now();
        let start_time = self.file_start_times.remove(&file_path)
            .unwrap_or(end_time);

        let transpile_time = end_time.duration_since(start_time);
        let input_size = input_content.len() as u64;
        let output_size = output_content.len() as u64;
        let line_count = input_content.lines().count() as u32;
        let character_count = input_content.chars().count() as u32;
        let complexity_score = self.calculate_complexity_score(input_content);

        let file_metric = FileMetrics {
            file_path: file_path.clone(),
            input_size_bytes: input_size,
            output_size_bytes: output_size,
            transpile_time_ms: transpile_time.as_millis() as u64,
            line_count,
            character_count,
            complexity_score,
        };

        self.metrics.push(file_metric);

        println!("📄 Analyzed: {} ({} bytes → {} bytes, {:.2}ms)",
            file_path.display(),
            input_size,
            output_size,
            transpile_time.as_millis()
        );

        Ok(())
    }

    pub fn finish_build(&mut self) -> Result<PerformanceMetrics, Box<dyn std::error::Error>> {
        let total_time = self.start_time
            .map(|start| start.elapsed())
            .unwrap_or(Duration::from_secs(0));

        let build_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let bundle_analysis = self.generate_bundle_analysis()?;
        let memory_usage = self.get_memory_usage();
        let system_info = self.get_system_info();

        let metrics = PerformanceMetrics {
            build_id,
            timestamp,
            total_build_time_ms: total_time.as_millis() as u64,
            file_metrics: self.metrics.clone(),
            bundle_analysis,
            memory_usage,
            system_info,
        };

        // Check against performance budgets
        self.check_performance_budgets(&metrics)?;

        println!("✅ Performance analysis completed in {:.2}ms", total_time.as_millis());

        Ok(metrics)
    }

    fn calculate_complexity_score(&self, content: &str) -> f64 {
        let mut score = 0.0;
        let lines: Vec<&str> = content.lines().collect();

        for line in &lines {
            let trimmed = line.trim();

            // Cyclomatic complexity indicators
            if trimmed.contains("whatever") || trimmed.contains("if") {
                score += 1.0;
            }
            if trimmed.contains("when") || trimmed.contains("while") {
                score += 1.0;
            }
            if trimmed.contains("since") || trimmed.contains("for") {
                score += 1.0;
            }
            if trimmed.contains("test") || trimmed.contains("try") {
                score += 1.0;
            }
            if trimmed.contains("grab") || trimmed.contains("catch") {
                score += 1.0;
            }

            // Nesting depth
            let indent_level = line.len() - line.trim_start().len();
            score += (indent_level / 4) as f64 * 0.1;

            // Function definitions
            if trimmed.contains("run ") {
                score += 0.5;
            }

            // Class definitions
            if trimmed.contains("model ") {
                score += 1.0;
            }
        }

        // Normalize by line count
        if !lines.is_empty() {
            score / lines.len() as f64
        } else {
            0.0
        }
    }

    fn generate_bundle_analysis(&self) -> Result<BundleAnalysis, Box<dyn std::error::Error>> {
        let total_input_size: u64 = self.metrics.iter().map(|m| m.input_size_bytes).sum();
        let total_output_size: u64 = self.metrics.iter().map(|m| m.output_size_bytes).sum();
        let compression_ratio = if total_input_size > 0 {
            total_output_size as f64 / total_input_size as f64
        } else {
            1.0
        };

        let dependency_graph = self.analyze_dependencies()?;
        let duplicate_detection = self.detect_duplicates()?;
        let largest_files = self.find_largest_files();
        let optimization_suggestions = self.generate_optimization_suggestions(&duplicate_detection, &largest_files);

        Ok(BundleAnalysis {
            total_input_size,
            total_output_size,
            compression_ratio,
            file_count: self.metrics.len() as u32,
            dependency_graph,
            duplicate_detection,
            largest_files,
            optimization_suggestions,
        })
    }

    fn analyze_dependencies(&self) -> Result<DependencyGraph, Box<dyn std::error::Error>> {
        let mut nodes = Vec::new();
        let edges = Vec::new();

        // Simple dependency analysis - in a real implementation, this would parse imports/exports
        for metric in &self.metrics {
            let node = DependencyNode {
                id: metric.file_path.to_string_lossy().to_string(),
                file_path: metric.file_path.clone(),
                size_bytes: metric.output_size_bytes,
                imports: vec![], // Would be parsed from actual file content
                exports: vec![], // Would be parsed from actual file content
            };
            nodes.push(node);
        }

        // Detect circular dependencies (simplified)
        let circular_dependencies = Vec::new(); // Would implement cycle detection

        Ok(DependencyGraph {
            nodes,
            edges,
            circular_dependencies,
        })
    }

    fn detect_duplicates(&self) -> Result<Vec<DuplicateFile>, Box<dyn std::error::Error>> {
        let mut duplicates = Vec::new();

        // Group files by size for potential duplicates
        let mut size_groups: HashMap<u64, Vec<&FileMetrics>> = HashMap::new();
        for metric in &self.metrics {
            size_groups.entry(metric.output_size_bytes)
                .or_default()
                .push(metric);
        }

        // Find groups with multiple files of the same size
        for (size, files) in size_groups {
            if files.len() > 1 {
                let duplicate = DuplicateFile {
                    content_hash: format!("size_{}", size), // Would use actual content hash
                    files: files.iter().map(|f| f.file_path.clone()).collect(),
                    size_bytes: size,
                    similarity_score: 1.0, // Would calculate actual similarity
                };
                duplicates.push(duplicate);
            }
        }

        Ok(duplicates)
    }

    fn find_largest_files(&self) -> Vec<FileSize> {
        let total_size: u64 = self.metrics.iter().map(|m| m.output_size_bytes).sum();
        let mut files: Vec<FileSize> = self.metrics.iter()
            .map(|m| FileSize {
                file_path: m.file_path.clone(),
                size_bytes: m.output_size_bytes,
                percentage_of_total: if total_size > 0 {
                    (m.output_size_bytes as f64 / total_size as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        files.truncate(10); // Top 10 largest files
        files
    }

    fn generate_optimization_suggestions(
        &self,
        duplicates: &[DuplicateFile],
        largest_files: &[FileSize]
    ) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Suggest removing duplicates
        for duplicate in duplicates {
            let potential_savings = duplicate.size_bytes * (duplicate.files.len() - 1) as u64;
            suggestions.push(OptimizationSuggestion {
                suggestion_type: OptimizationType::RemoveDuplicates,
                description: format!(
                    "Remove {} duplicate files with {} bytes each",
                    duplicate.files.len() - 1,
                    duplicate.size_bytes
                ),
                potential_savings_bytes: potential_savings,
                priority: if potential_savings > 10_000 {
                    OptimizationPriority::High
                } else {
                    OptimizationPriority::Medium
                },
                files_affected: duplicate.files.clone(),
            });
        }

        // Suggest code splitting for large files
        for file in largest_files.iter().take(3) {
            if file.size_bytes > 50_000 {
                suggestions.push(OptimizationSuggestion {
                    suggestion_type: OptimizationType::CodeSplitting,
                    description: format!(
                        "Consider splitting large file {} ({} bytes, {:.1}% of total)",
                        file.file_path.display(),
                        file.size_bytes,
                        file.percentage_of_total
                    ),
                    potential_savings_bytes: file.size_bytes / 2, // Estimated
                    priority: OptimizationPriority::Medium,
                    files_affected: vec![file.file_path.clone()],
                });
            }
        }

        // Suggest tree shaking if bundle is large
        let total_size: u64 = self.metrics.iter().map(|m| m.output_size_bytes).sum();
        if total_size > 500_000 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: OptimizationType::TreeShaking,
                description: "Enable tree shaking to remove unused code".to_string(),
                potential_savings_bytes: total_size / 10, // Estimated 10% savings
                priority: OptimizationPriority::High,
                files_affected: self.metrics.iter().map(|m| m.file_path.clone()).collect(),
            });
        }

        suggestions
    }

    fn get_memory_usage(&self) -> Option<MemoryUsage> {
        // In a real implementation, this would track actual memory usage
        Some(MemoryUsage {
            peak_memory_mb: 64.0,
            average_memory_mb: 32.0,
            garbage_collections: 0,
        })
    }

    fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get() as u32,
            total_memory_mb: 8192, // Would get actual system memory
            node_version: None, // Would detect if Node.js is available
            rust_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    fn check_performance_budgets(&self, metrics: &PerformanceMetrics) -> Result<(), Box<dyn std::error::Error>> {
        // Use default performance budgets since they're not in simplified config
        let default_bundle_size_limit = 1024 * 1024; // 1MB
        let default_load_time_budget = 3000; // 3 seconds

        // Check bundle size limit
        if metrics.bundle_analysis.total_output_size > default_bundle_size_limit {
            println!("⚠️  Bundle size exceeds limit: {} bytes (limit: {} bytes)",
                metrics.bundle_analysis.total_output_size,
                default_bundle_size_limit
            );
        }

        // Check build time budget
        if metrics.total_build_time_ms > default_load_time_budget as u64 {
            println!("⚠️  Build time exceeds budget: {}ms (budget: {}ms)",
                metrics.total_build_time_ms,
                default_load_time_budget
            );
        }

        Ok(())
    }

    pub fn generate_report(&self, metrics: &PerformanceMetrics, format: &str) -> Result<String, Box<dyn std::error::Error>> {
        match format {
            "html" => self.generate_html_report(metrics),
            "json" => Ok(serde_json::to_string_pretty(metrics)?),
            "markdown" => self.generate_markdown_report(metrics),
            _ => self.generate_text_report(metrics),
        }
    }

    fn generate_html_report(&self, metrics: &PerformanceMetrics) -> Result<String, Box<dyn std::error::Error>> {
        let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>NullScript Performance Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }}
        .container {{ max-width: 1200px; margin: 0 auto; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
        .header {{ text-align: center; margin-bottom: 40px; }}
        .metric-card {{ background: #f9f9f9; padding: 20px; margin: 10px 0; border-radius: 5px; }}
        .metric-title {{ font-weight: bold; color: #333; margin-bottom: 10px; }}
        .metric-value {{ font-size: 24px; color: #007acc; }}
        .files-table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
        .files-table th, .files-table td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
        .files-table th {{ background: #007acc; color: white; }}
        .suggestion {{ background: #fff3cd; padding: 15px; margin: 10px 0; border-left: 4px solid #ffc107; }}
        .priority-high {{ border-left-color: #dc3545; }}
        .priority-medium {{ border-left-color: #ffc107; }}
        .priority-low {{ border-left-color: #28a745; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎭 NullScript Performance Report</h1>
            <p>Build ID: {}</p>
            <p>Generated: {}</p>
        </div>

        <div class="metric-card">
            <div class="metric-title">Total Build Time</div>
            <div class="metric-value">{}ms</div>
        </div>

        <div class="metric-card">
            <div class="metric-title">Bundle Size</div>
            <div class="metric-value">{} bytes ({} files)</div>
        </div>

        <div class="metric-card">
            <div class="metric-title">Compression Ratio</div>
            <div class="metric-value">{:.2}%</div>
        </div>

        <h2>📄 File Metrics</h2>
        <table class="files-table">
            <tr>
                <th>File</th>
                <th>Input Size</th>
                <th>Output Size</th>
                <th>Transpile Time</th>
                <th>Complexity</th>
            </tr>
            {}
        </table>

        <h2>💡 Optimization Suggestions</h2>
        {}

        <h2>📊 System Information</h2>
        <div class="metric-card">
            <p><strong>OS:</strong> {} ({})</p>
            <p><strong>CPU Cores:</strong> {}</p>
            <p><strong>Memory:</strong> {} MB</p>
            <p><strong>NullScript Version:</strong> {}</p>
        </div>
    </div>
</body>
</html>
"#,
            metrics.build_id,
            chrono::DateTime::from_timestamp(metrics.timestamp as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S"),
            metrics.total_build_time_ms,
            metrics.bundle_analysis.total_output_size,
            metrics.bundle_analysis.file_count,
            metrics.bundle_analysis.compression_ratio * 100.0,
            metrics.file_metrics.iter()
                .map(|f| format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}ms</td><td>{:.2}</td></tr>",
                    f.file_path.display(),
                    f.input_size_bytes,
                    f.output_size_bytes,
                    f.transpile_time_ms,
                    f.complexity_score
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            metrics.bundle_analysis.optimization_suggestions.iter()
                .map(|s| {
                    let priority_class = match s.priority {
                        OptimizationPriority::High => "priority-high",
                        OptimizationPriority::Medium => "priority-medium",
                        _ => "priority-low",
                    };
                    format!(
                        "<div class=\"suggestion {}\"><strong>{:?}:</strong> {} (Save {} bytes)</div>",
                        priority_class,
                        s.suggestion_type,
                        s.description,
                        s.potential_savings_bytes
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
            metrics.system_info.os,
            metrics.system_info.arch,
            metrics.system_info.cpu_cores,
            metrics.system_info.total_memory_mb,
            metrics.system_info.rust_version
        );

        Ok(html)
    }

    fn generate_markdown_report(&self, metrics: &PerformanceMetrics) -> Result<String, Box<dyn std::error::Error>> {
        let mut md = String::new();

        md.push_str("# 🎭 NullScript Performance Report\n\n");
        md.push_str(&format!("**Build ID:** {}\n", metrics.build_id));
        md.push_str(&format!("**Generated:** {}\n\n",
            chrono::DateTime::from_timestamp(metrics.timestamp as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S")
        ));

        md.push_str("## 📊 Summary\n\n");
        md.push_str(&format!("- **Total Build Time:** {}ms\n", metrics.total_build_time_ms));
        md.push_str(&format!("- **Bundle Size:** {} bytes ({} files)\n",
            metrics.bundle_analysis.total_output_size,
            metrics.bundle_analysis.file_count
        ));
        md.push_str(&format!("- **Compression Ratio:** {:.2}%\n\n",
            metrics.bundle_analysis.compression_ratio * 100.0
        ));

        md.push_str("## 📄 File Metrics\n\n");
        md.push_str("| File | Input Size | Output Size | Transpile Time | Complexity |\n");
        md.push_str("|------|------------|-------------|----------------|------------|\n");

        for file in &metrics.file_metrics {
            md.push_str(&format!(
                "| {} | {} | {} | {}ms | {:.2} |\n",
                file.file_path.display(),
                file.input_size_bytes,
                file.output_size_bytes,
                file.transpile_time_ms,
                file.complexity_score
            ));
        }

        md.push_str("\n## 💡 Optimization Suggestions\n\n");
        for suggestion in &metrics.bundle_analysis.optimization_suggestions {
            let priority_emoji = match suggestion.priority {
                OptimizationPriority::High => "🔴",
                OptimizationPriority::Medium => "🟡",
                _ => "🟢",
            };
            md.push_str(&format!(
                "- {} **{:?}:** {} (Save {} bytes)\n",
                priority_emoji,
                suggestion.suggestion_type,
                suggestion.description,
                suggestion.potential_savings_bytes
            ));
        }

        Ok(md)
    }

    fn generate_text_report(&self, metrics: &PerformanceMetrics) -> Result<String, Box<dyn std::error::Error>> {
        let mut report = String::new();

        report.push_str("🎭 NullScript Performance Report\n");
        report.push_str("================================\n\n");

        report.push_str(&format!("Build ID: {}\n", metrics.build_id));
        report.push_str(&format!("Generated: {}\n\n",
            chrono::DateTime::from_timestamp(metrics.timestamp as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S")
        ));

        report.push_str("📊 Summary:\n");
        report.push_str(&format!("  Total Build Time: {}ms\n", metrics.total_build_time_ms));
        report.push_str(&format!("  Bundle Size: {} bytes ({} files)\n",
            metrics.bundle_analysis.total_output_size,
            metrics.bundle_analysis.file_count
        ));
        report.push_str(&format!("  Compression Ratio: {:.2}%\n\n",
            metrics.bundle_analysis.compression_ratio * 100.0
        ));

        report.push_str("📄 File Metrics:\n");
        for file in &metrics.file_metrics {
            report.push_str(&format!(
                "  {} - {} bytes → {} bytes ({}ms, complexity: {:.2})\n",
                file.file_path.display(),
                file.input_size_bytes,
                file.output_size_bytes,
                file.transpile_time_ms,
                file.complexity_score
            ));
        }

        report.push_str("\n💡 Optimization Suggestions:\n");
        for suggestion in &metrics.bundle_analysis.optimization_suggestions {
            report.push_str(&format!(
                "  {:?}: {} (Save {} bytes)\n",
                suggestion.suggestion_type,
                suggestion.description,
                suggestion.potential_savings_bytes
            ));
        }

        Ok(report)
    }

    pub async fn save_report(&self, metrics: &PerformanceMetrics, output_dir: &Path, format: &str) -> Result<(), Box<dyn std::error::Error>> {
        let report_content = self.generate_report(metrics, format)?;

        let filename = format!("performance-report-{}.{}",
            metrics.build_id,
            match format {
                "html" => "html",
                "json" => "json",
                "markdown" => "md",
                _ => "txt",
            }
        );

        let report_path = output_dir.join(filename);
        tokio::fs::create_dir_all(output_dir).await?;
        tokio::fs::write(&report_path, report_content).await?;

        println!("📊 Performance report saved: {}", report_path.display());
        Ok(())
    }
}
