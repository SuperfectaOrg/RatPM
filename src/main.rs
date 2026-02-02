use anyhow::Result;
use clap::Parser;

mod backend;
mod cli;
mod config;
mod core;

use cli::Cli;
use core::context::Context;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .without_time()
        .init();

    let cli = Cli::parse();

    let config = config::load_config()?;

    let mut context = Context::new(config)?;

    if let Err(e) = cli.execute(&mut context) {
        let exit_code = if let Some(ratpm_err) = e.downcast_ref::<core::errors::RatpmError>() {
            ratpm_err.exit_code()
        } else {
            1
        };

        eprintln!("Error: {}", e);
        std::process::exit(exit_code);
    }

    Ok(())
}
