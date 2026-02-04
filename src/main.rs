mod cli;
mod core;
mod error;
mod resolver;
mod sources;
mod output;
mod utils;

use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = core::engine::run(args).await {
        eprintln!("[!] {}", e);
        std::process::exit(1);
    }
}
