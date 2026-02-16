use std::env;
use std::process;

use pgdoctor::Config;

mod cli;

#[tokio::main]
async fn main() {
    // .env file overrides shell variables when present (for development)
    let _ = dotenvy::dotenv_override();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cli::print_usage();
        process::exit(2);
    }

    let cfg: Config = Config::load();

    let result: Result<(), anyhow::Error> = match args[1].as_str() {
        "snapshot" => cli::run_snapshot(&cfg).await,
        "upload" => cli::run_upload(&cfg).await,
        "--version" | "-v" | "version" => {
            cli::print_version();
            return;
        }
        cmd => {
            eprintln!("unknown command: {}", cmd);
            cli::print_usage();
            process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
