use std::path::PathBuf;
use std::process::Command;

pub struct CommandUtils;

impl CommandUtils {
    pub fn execute_command(command: &str, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        Command::new(command).args(args).output()
    }

    pub fn execute_node(script_path: &PathBuf) -> Result<std::process::Output, std::io::Error> {
        Self::execute_command("node", &[&script_path.to_string_lossy()])
    }

    pub fn execute_tsc(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        Self::execute_command("tsc", args)
    }

    pub fn execute_tsc_no_emit(project_path: &str) -> Result<std::process::Output, std::io::Error> {
        Self::execute_tsc(&["--noEmit", "--project", project_path])
    }

    pub fn execute_tsc_in_dir(args: &[&str], working_dir: &PathBuf) -> Result<std::process::Output, std::io::Error> {
        Command::new("tsc")
            .args(args)
            .current_dir(working_dir)
            .output()
    }
}
