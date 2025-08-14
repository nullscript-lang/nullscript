use notify::{RecommendedWatcher, RecursiveMode, Watcher, Result as NotifyResult, Event, EventKind};
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::sleep;
use crate::config::loader::NullScriptConfig;

#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub kind: ChangeKind,
}

#[derive(Debug, Clone)]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
}

pub struct TerminalWatcher {
    watchers: Vec<RecommendedWatcher>,
    change_receiver: Receiver<FileChangeEvent>,
    change_sender: Sender<FileChangeEvent>,
    debounce_timeout: Duration,
    last_changes: HashMap<PathBuf, Instant>,
    run_on_save: bool,
}

impl TerminalWatcher {
    pub fn new(run_on_save: bool) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            watchers: Vec::new(),
            change_receiver: receiver,
            change_sender: sender,
            debounce_timeout: Duration::from_millis(300),
            last_changes: HashMap::new(),
            run_on_save,
        }
    }

    pub async fn start(&mut self, watch_paths: Vec<PathBuf>, ignore_patterns: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        println!("👀 Starting NullScript Terminal Watcher");

        // Start file watchers
        self.start_watchers(watch_paths, ignore_patterns)?;

        // Start change processing loop
        self.start_change_processing().await;

        Ok(())
    }

    fn start_watchers(&mut self, watch_paths: Vec<PathBuf>, ignore_patterns: Vec<String>) -> NotifyResult<()> {
        for watch_path in watch_paths {
            let sender = self.change_sender.clone();
            let ignore_patterns = ignore_patterns.clone();

            let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
                match res {
                    Ok(event) => {
                        if let Err(e) = Self::handle_file_event(event, &sender, &ignore_patterns) {
                            eprintln!("Error handling file event: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            })?;

            watcher.watch(&watch_path, RecursiveMode::Recursive)?;
            self.watchers.push(watcher);

            println!("📁 Watching: {}", watch_path.display());
        }

        Ok(())
    }

    fn handle_file_event(
        event: Event,
        sender: &Sender<FileChangeEvent>,
        ignore_patterns: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for path in &event.paths {
            // Check if path should be ignored
            if Self::should_ignore_path(path, ignore_patterns) {
                continue;
            }

            // Only process .ns files
            if let Some(extension) = path.extension() {
                if extension != "ns" {
                    continue;
                }
            } else {
                continue;
            }

            let change_kind = match event.kind {
                EventKind::Create(_) => ChangeKind::Created,
                EventKind::Modify(_) => ChangeKind::Modified,
                EventKind::Remove(_) => ChangeKind::Deleted,
                _ => continue,
            };

            let file_event = FileChangeEvent {
                path: path.clone(),
                kind: change_kind,
            };

            sender.send(file_event)?;
        }

        Ok(())
    }

    fn should_ignore_path(path: &Path, ignore_patterns: &[String]) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in ignore_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }

        // Always ignore hidden files and common editor temporary files
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            if filename_str.starts_with('.')
                || filename_str.ends_with('~')
                || filename_str.ends_with(".tmp")
                || filename_str.ends_with(".swp") {
                return true;
            }
        }

        false
    }

    async fn start_change_processing(&mut self) {
        println!("🚀 Watcher ready. Waiting for file changes...");
        println!("💡 Press Ctrl+C to stop watching\n");

        loop {
            if let Ok(change) = self.change_receiver.try_recv() {
                let now = Instant::now();

                // Debounce: ignore rapid successive changes to the same file
                if let Some(&last_change) = self.last_changes.get(&change.path) {
                    if now.duration_since(last_change) < self.debounce_timeout {
                        continue;
                    }
                }

                self.last_changes.insert(change.path.clone(), now);
                self.handle_nullscript_change(change).await;
            }

            // Use a shorter sleep for more responsive watching
            sleep(Duration::from_millis(50)).await;
        }
    }

    async fn handle_nullscript_change(&self, change: FileChangeEvent) {
        match change.kind {
            ChangeKind::Modified | ChangeKind::Created => {
                println!("📝 File changed: {}", change.path.display());

                // Trigger transpilation
                if let Err(e) = self.transpile_file(&change.path).await {
                    eprintln!("❌ Transpilation error: {}", e);
                } else {
                    println!("✅ Transpiled successfully");

                    // Run the file if requested
                    if self.run_on_save {
                        if let Err(e) = self.run_file(&change.path).await {
                            eprintln!("❌ Execution error: {}", e);
                        }
                    }
                }

                println!(); // Add spacing for readability
            }
            ChangeKind::Deleted => {
                println!("🗑️  File deleted: {}", change.path.display());
                self.cleanup_output(&change.path).await;
                println!();
            }
        }
    }

    async fn transpile_file(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use crate::compiler::transpiler::NullScriptTranspiler;
        use std::fs;

        let input_content = fs::read_to_string(file_path)?;
        let transpiler = NullScriptTranspiler::new();

        let output_content = transpiler.transpile(&input_content)?;

        // Determine output path
        let mut output_path = file_path.with_extension("js");
        if let Some(parent) = file_path.parent() {
            if parent.file_name().unwrap_or_default() == "src" {
                // Move from src/ to dist/
                output_path = parent.parent()
                    .unwrap_or(Path::new("."))
                    .join("dist")
                    .join(file_path.file_name().unwrap())
                    .with_extension("js");
            }
        }

        // Ensure output directory exists
        if let Some(output_dir) = output_path.parent() {
            fs::create_dir_all(output_dir)?;
        }

        fs::write(&output_path, output_content)?;
        println!("📄 Output: {}", output_path.display());

        Ok(())
    }

    async fn run_file(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;

        // Find the corresponding JS file
        let mut js_path = file_path.with_extension("js");
        if let Some(parent) = file_path.parent() {
            if parent.file_name().unwrap_or_default() == "src" {
                js_path = parent.parent()
                    .unwrap_or(Path::new("."))
                    .join("dist")
                    .join(file_path.file_name().unwrap())
                    .with_extension("js");
            }
        }

        if !js_path.exists() {
            return Err("Transpiled JS file not found".into());
        }

        println!("🚀 Running: {}", js_path.display());

        let output = Command::new("node")
            .arg(&js_path)
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                println!("📤 Output:\n{}", stdout);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("❌ Error:\n{}", stderr);
        }

        Ok(())
    }

    async fn cleanup_output(&self, file_path: &Path) {
        let mut output_path = file_path.with_extension("js");
        if let Some(parent) = file_path.parent() {
            if parent.file_name().unwrap_or_default() == "src" {
                output_path = parent.parent()
                    .unwrap_or(Path::new("."))
                    .join("dist")
                    .join(file_path.file_name().unwrap())
                    .with_extension("js");
            }
        }

        if output_path.exists() {
            if let Err(e) = std::fs::remove_file(&output_path) {
                eprintln!("Warning: Could not remove {}: {}", output_path.display(), e);
            } else {
                println!("🗑️  Cleaned up: {}", output_path.display());
            }
        }
    }
}

pub struct TerminalDevCommand {
    config: NullScriptConfig,
    watcher: Option<TerminalWatcher>,
}

impl TerminalDevCommand {
    pub fn new(config: NullScriptConfig) -> Self {
        Self {
            config,
            watcher: None,
        }
    }

    pub async fn start(&mut self, watch_paths: Vec<PathBuf>, run_on_save: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎭 NullScript Terminal Development Mode");
        println!("=====================================");

        let mut watcher = TerminalWatcher::new(run_on_save);
        let ignore_patterns = self.config.get_exclude_patterns();

        // This will run indefinitely - the infinite loop is in start_change_processing
        watcher.start(watch_paths, ignore_patterns).await?;
        self.watcher = Some(watcher);

        // This should never be reached due to the infinite loop above
        Ok(())
    }
}
