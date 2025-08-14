use std::path::Path;
use std::process::Command;

pub struct CommandUtils;

impl CommandUtils {
    pub fn execute_command(command: &str, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        Command::new(command).args(args).output()
    }

    pub fn execute_node(script_path: &Path) -> Result<std::process::Output, std::io::Error> {
        Self::execute_command("node", &[&script_path.to_string_lossy()])
    }


}
