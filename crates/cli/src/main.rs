use std::process;

use colored::Colorize;

#[tokio::main]
async fn main() {
    if let Err(e) = carton::run_cli().await {
        eprintln!("Error: {}", e.to_string().bold().red());
        process::exit(1);
    }
}
