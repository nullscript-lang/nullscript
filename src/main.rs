mod cli;
mod core;
mod compiler;
mod utils;

use cli::run;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let program_path = args.get(0).map(|s| s.as_str()).unwrap_or("");

    let is_nullscript_command = if let Some(file_name) = std::path::Path::new(program_path).file_name() {
        file_name.to_string_lossy() == "nullscript"
    } else {
        false
    };

    if is_nullscript_command {
        if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
            println!("nullscript v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        println!("🎭 NullScript Transpiler v{}", env!("CARGO_PKG_VERSION"));
        println!("==================================================");
        println!("🚀 To transpile NullScript code, use the 'nsc' command.");
        println!();
        println!("📝 Examples:");
        println!("   nsc --help                   # Show help");
        println!("   nsc keywords                 # Show all keywords");
        println!("   nsc build src/               # Transpile files");
        println!("   nsc run hello.ns             # Run a file");
        println!();
        println!("==================================================");
        std::process::exit(0);
    }

    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
