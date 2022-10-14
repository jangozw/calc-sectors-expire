mod cli;
use std::env;
use clap::Parser;
// use tracing::{error, info, warn, debug,trace};
use tracing::*;


fn main() {
    initialize_logger(3);
    let app = cli::CLI::parse();
    match app.start() {
        Ok(_) => {}
        Err(e) => {
            error!("process exit with err: {:?}", e)
        }
    }
}

fn initialize_logger(verbosity: u8) {
    match verbosity {
        0 => env::set_var("RUST_LOG", "info"),
        1 => env::set_var("RUST_LOG", "debug"),
        2 | 3 => env::set_var("RUST_LOG", "trace"),
        _ => env::set_var("RUST_LOG", "info"),
    };

    // Filter out undesirable logs.
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("hyper::client=off".parse().unwrap())
        .add_directive("hyper::proto=off".parse().unwrap())
        .add_directive("jsonrpsee=off".parse().unwrap())
        .add_directive("mio=off".parse().unwrap())
        .add_directive("rusoto_core=off".parse().unwrap())
        .add_directive("tokio_util=off".parse().unwrap())
        .add_directive("want=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());

    // Initialize tracing.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(verbosity == 3)
        .try_init();
}