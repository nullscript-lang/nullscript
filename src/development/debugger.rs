use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Write;
use tokio::time::{sleep, Duration};
use crate::config::loader::NullScriptConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: u32,
    pub file_path: PathBuf,
    pub line: u32,
    pub condition: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSession {
    pub session_id: String,
    pub target_file: PathBuf,
    pub breakpoints: Vec<Breakpoint>,
    pub current_location: Option<DebugLocation>,
    pub variables: HashMap<String, DebugVariable>,
    pub call_stack: Vec<StackFrame>,
    pub performance_data: Option<PerformanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLocation {
    pub file_path: PathBuf,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugVariable {
    pub name: String,
    pub value: String,
    pub type_info: String,
    pub nullscript_keyword: Option<String>, // Original NullScript keyword if applicable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: String,
    pub file_path: PathBuf,
    pub line: u32,
    pub column: u32,
    pub nullscript_function: Option<String>, // Original NullScript function name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    pub execution_time_ms: u64,
    pub memory_usage_mb: f64,
    pub function_calls: Vec<FunctionCall>,
    pub performance_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub function_name: String,
    pub execution_time_ms: u64,
    pub call_count: u32,
}

pub struct NullScriptDebugger {
    session: Option<DebugSession>,
    source_map: SourceMap,
}

#[derive(Debug, Clone)]
pub struct SourceMap {
    pub mappings: HashMap<PathBuf, FileMapping>,
}

#[derive(Debug, Clone)]
pub struct FileMapping {
    // Simplified - only keeping essential mapping structure
}

impl NullScriptDebugger {
    pub fn new(_config: NullScriptConfig) -> Self {
        Self {
            session: None,
            source_map: SourceMap {
                mappings: HashMap::new(),
            },
        }
    }

    pub async fn start_debug_session(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("🐛 Starting NullScript Debug Session");
        println!("=====================================");
        println!("📄 Target file: {}", file_path.display());

        // First, transpile the NullScript file and generate source maps
        let js_file = self.transpile_with_source_maps(&file_path).await?;

        // Create debug session
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = DebugSession {
            session_id: session_id.clone(),
            target_file: file_path.clone(),
            breakpoints: Vec::new(),
            current_location: None,
            variables: HashMap::new(),
            call_stack: Vec::new(),
            performance_data: None,
        };

        self.session = Some(session);

        println!("🔧 Generated JavaScript: {}", js_file.display());
        println!("🗺️  Source maps: Generated");
        println!("🎯 Debug session ID: {}", session_id);

        // Check if Node.js supports debugging
        if !self.check_node_debug_support().await? {
            return Err("Node.js debugging not available".into());
        }

        println!("✅ Debug environment ready");

        // Start interactive debugging session
        self.start_interactive_session(js_file).await?;

        Ok(())
    }

    async fn transpile_with_source_maps(&mut self, file_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        use crate::compiler::transpiler::NullScriptTranspiler;
        use std::fs;

        let input_content = fs::read_to_string(file_path)?;
        let transpiler = NullScriptTranspiler::new();

        // Validate syntax first
        transpiler.validate_syntax(&input_content, Some(file_path))?;

        // Transpile to JavaScript
        let output_content = transpiler.transpile(&input_content)?;

        // Generate output path
        let js_file = file_path.with_extension("debug.js");

        // Add source mapping comment
        let source_map_content = self.generate_source_map(file_path, &input_content, &output_content)?;
        let js_with_source_map = format!("{}\n//# sourceMappingURL={}.map",
            output_content,
            js_file.file_name().unwrap().to_string_lossy()
        );

        // Write JavaScript file
        fs::write(&js_file, js_with_source_map)?;

        // Write source map file
        let source_map_file = js_file.with_extension("debug.js.map");
        fs::write(&source_map_file, source_map_content)?;

        Ok(js_file)
    }

    fn generate_source_map(&mut self, ns_file: &Path, ns_content: &str, js_content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let ns_lines: Vec<&str> = ns_content.lines().collect();
        let js_lines: Vec<&str> = js_content.lines().collect();

        let mut line_mappings = HashMap::new();
        let mut function_mappings = HashMap::new();

        // Simple line mapping (1:1 for now, could be more sophisticated)
        for (js_line_idx, js_line) in js_lines.iter().enumerate() {
            if js_line_idx < ns_lines.len() {
                line_mappings.insert((js_line_idx + 1) as u32, (js_line_idx + 1) as u32);
            }

            // Map function names
            if js_line.contains("function ") {
                if let Some(ns_line) = ns_lines.get(js_line_idx) {
                    if ns_line.contains("run ") {
                        // Extract function names and map them
                        if let (Some(js_func), Some(ns_func)) = (
                            self.extract_js_function_name(js_line),
                            self.extract_ns_function_name(ns_line)
                        ) {
                            function_mappings.insert(js_func, ns_func);
                        }
                    }
                }
            }
        }

        let file_mapping = FileMapping {
            // Simplified mapping structure
        };

        self.source_map.mappings.insert(ns_file.to_path_buf(), file_mapping);

        // Generate basic source map JSON
        let source_map = serde_json::json!({
            "version": 3,
            "file": ns_file.with_extension("debug.js").file_name(),
            "sourceRoot": "",
            "sources": [ns_file.file_name()],
            "names": [],
            "mappings": "AAAA" // Simplified mapping
        });

        Ok(serde_json::to_string_pretty(&source_map)?)
    }

    fn extract_js_function_name(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find("function ") {
            let remaining = &line[start + 9..];
            if let Some(end) = remaining.find('(') {
                return Some(remaining[..end].trim().to_string());
            }
        }
        None
    }

    fn extract_ns_function_name(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find("run ") {
            let remaining = &line[start + 4..];
            if let Some(end) = remaining.find('(') {
                return Some(remaining[..end].trim().to_string());
            }
        }
        None
    }

    async fn check_node_debug_support(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let output = Command::new("node")
            .arg("--version")
            .output()?;

        if !output.status.success() {
            return Ok(false);
        }

        let version_str = String::from_utf8_lossy(&output.stdout);
        println!("🟢 Node.js version: {}", version_str.trim());

        // Check if Node.js supports inspector protocol
        let inspector_check = Command::new("node")
            .arg("--help")
            .output()?;

        let help_text = String::from_utf8_lossy(&inspector_check.stdout);
        Ok(help_text.contains("--inspect"))
    }

    async fn start_interactive_session(&mut self, js_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🎮 Interactive Debug Session");
        println!("============================");
        println!("Commands:");
        println!("  b <line>     - Set breakpoint at line");
        println!("  c            - Continue execution");
        println!("  s            - Step into");
        println!("  n            - Step over");
        println!("  l            - List source code");
        println!("  v            - Show variables");
        println!("  stack        - Show call stack");
        println!("  perf         - Show performance data");
        println!("  q            - Quit debugger");
        println!();

        // Start Node.js with debugging enabled
        let mut child = Command::new("node")
            .arg("--inspect-brk")
            .arg(&js_file)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        println!("🚀 Started Node.js debugger (PID: {})", child.id());

        // Give Node.js time to start
        sleep(Duration::from_millis(1000)).await;

        // Start command loop
        loop {
            print!("🐛 debug> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let command = input.trim();

            match command {
                "q" | "quit" => {
                    println!("🛑 Ending debug session...");
                    child.kill()?;
                    break;
                }
                "c" | "continue" => {
                    println!("▶️  Continuing execution...");
                    // In a real implementation, this would send continue command to debugger
                }
                "l" | "list" => {
                    self.show_source_code().await?;
                }
                "v" | "variables" => {
                    self.show_variables().await?;
                }
                "stack" => {
                    self.show_call_stack().await?;
                }
                "perf" => {
                    self.show_performance_data().await?;
                }
                cmd if cmd.starts_with("b ") => {
                    if let Ok(line) = cmd[2..].trim().parse::<u32>() {
                        self.set_breakpoint(line).await?;
                    } else {
                        println!("❌ Invalid line number");
                    }
                }
                "" => {
                    // Empty command, do nothing
                }
                _ => {
                    println!("❓ Unknown command. Type 'q' to quit.");
                }
            }
        }

        Ok(())
    }

    async fn show_source_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = &self.session {
            let content = std::fs::read_to_string(&session.target_file)?;
            let lines: Vec<&str> = content.lines().collect();

            println!("\n📄 Source Code: {}", session.target_file.display());
            println!("{}", "─".repeat(50));

            for (idx, line) in lines.iter().enumerate() {
                let line_num = idx + 1;
                let has_breakpoint = session.breakpoints.iter()
                    .any(|bp| bp.line == line_num as u32 && bp.enabled);

                let prefix = if has_breakpoint { "🔴" } else { "  " };
                println!("{} {:3}: {}", prefix, line_num, line);
            }
            println!();
        }
        Ok(())
    }

    async fn show_variables(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n📋 Variables (NullScript View)");
        println!("{}", "─".repeat(30));

        if let Some(session) = &self.session {
            if session.variables.is_empty() {
                println!("No variables in current scope");
            } else {
                for (name, var) in &session.variables {
                    let display_name = if let Some(ns_keyword) = &var.nullscript_keyword {
                        format!("{} ({})", name, ns_keyword)
                    } else {
                        name.clone()
                    };

                    println!("  {} = {} ({})", display_name, var.value, var.type_info);
                }
            }
        } else {
            println!("No active debug session");
        }
        println!();
        Ok(())
    }

    async fn show_call_stack(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n📚 Call Stack (NullScript View)");
        println!("{}", "─".repeat(35));

        if let Some(session) = &self.session {
            if session.call_stack.is_empty() {
                println!("No call stack available");
            } else {
                for (idx, frame) in session.call_stack.iter().enumerate() {
                    let function_name = frame.nullscript_function.as_ref()
                        .unwrap_or(&frame.function_name);

                    println!("  {}: {} at {}:{}",
                        idx,
                        function_name,
                        frame.file_path.display(),
                        frame.line
                    );
                }
            }
        } else {
            println!("No active debug session");
        }
        println!();
        Ok(())
    }

    async fn show_performance_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n⚡ Performance Data");
        println!("{}", "─".repeat(20));

        if let Some(session) = &self.session {
            if let Some(perf) = &session.performance_data {
                println!("📊 Execution time: {}ms", perf.execution_time_ms);
                println!("💾 Memory usage: {:.2}MB", perf.memory_usage_mb);

                if !perf.function_calls.is_empty() {
                    println!("\n🔧 Function Performance:");
                    for func_call in &perf.function_calls {
                        println!("  {} - {}ms ({} calls)",
                            func_call.function_name,
                            func_call.execution_time_ms,
                            func_call.call_count
                        );
                    }
                }

                if !perf.performance_hints.is_empty() {
                    println!("\n💡 Performance Hints:");
                    for hint in &perf.performance_hints {
                        println!("  • {}", hint);
                    }
                }
            } else {
                println!("No performance data available");
            }
        } else {
            println!("No active debug session");
        }
        println!();
        Ok(())
    }

    async fn set_breakpoint(&mut self, line: u32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = &mut self.session {
            let breakpoint = Breakpoint {
                id: session.breakpoints.len() as u32 + 1,
                file_path: session.target_file.clone(),
                line,
                condition: None,
                enabled: true,
            };

            session.breakpoints.push(breakpoint);
            println!("🔴 Breakpoint set at line {}", line);
        }
        Ok(())
    }

    // Unused debugging helper methods removed - will be implemented when needed
}
