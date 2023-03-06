use std::process;

use carton;

#[tokio::main]
async fn main() {
    if let Err(e) = carton::run_cli().await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
