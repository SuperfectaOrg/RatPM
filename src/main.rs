use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod core;
mod backend;

use cli::Cli;
use core::context::Context;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .with_target(false)
        .without_time()
        .init();

    let cli = Cli::parse();

    let config = config::load_config()?;
    
    let mut context = Context::new(config)?;
    
    cli.execute(&mut context)?;

    Ok(())
}
