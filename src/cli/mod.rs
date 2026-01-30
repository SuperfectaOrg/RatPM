use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
pub mod output;

use crate::core::context::Context;

#[derive(Parser)]
#[command(name = "ratpm")]
#[command(about = "RatOS Package Manager", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    assume_yes: bool,

    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        packages: Vec<String>,
    },
    Remove {
        packages: Vec<String>,
    },
    Update,
    Upgrade {
        packages: Option<Vec<String>>,
    },
    Search {
        query: String,
    },
    Info {
        package: String,
    },
    List {
        #[arg(long)]
        installed: bool,
        #[arg(long)]
        available: bool,
    },
    Sync,
    Doctor,
    History {
        #[arg(short, long)]
        limit: Option<usize>,
    },
}

impl Cli {
    pub fn execute(self, context: &mut Context) -> Result<()> {
        if self.assume_yes {
            context.set_assume_yes(true);
        }
        
        if self.no_color {
            context.set_color(false);
        }

        match self.command {
            Commands::Install { packages } => {
                commands::install::execute(context, packages)
            }
            Commands::Remove { packages } => {
                commands::remove::execute(context, packages)
            }
            Commands::Update => {
                commands::update::execute(context)
            }
            Commands::Upgrade { packages } => {
                commands::upgrade::execute(context, packages)
            }
            Commands::Search { query } => {
                commands::search::execute(context, &query)
            }
            Commands::Info { package } => {
                commands::info::execute(context, &package)
            }
            Commands::List { installed, available } => {
                commands::list::execute(context, installed, available)
            }
            Commands::Sync => {
                commands::sync::execute(context)
            }
            Commands::Doctor => {
                commands::doctor::execute(context)
            }
            Commands::History { limit } => {
                commands::history::execute(context, limit)
            }
        }
    }
}
