use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod commands;
pub mod output;

use crate::core::context::Context;

#[derive(Parser)]
#[command(name = "ratpm")]
#[command(version)]
#[command(about = "RatOS Package Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Assume yes to all prompts
    #[arg(short = 'y', long, global = true)]
    pub assume_yes: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install packages
    #[command(name = "install")]
    Install {
        /// Packages to install
        #[arg(required = true)]
        packages: Vec<String>,
    },

    /// Remove packages
    #[command(name = "remove")]
    Remove {
        /// Packages to remove
        #[arg(required = true)]
        packages: Vec<String>,
    },

    /// Update repository metadata
    #[command(name = "update")]
    Update,

    /// Upgrade installed packages
    #[command(name = "upgrade")]
    Upgrade {
        /// Specific packages to upgrade (all if omitted)
        packages: Option<Vec<String>>,
    },

    /// Search for packages
    #[command(name = "search")]
    Search {
        /// Search query
        query: String,
    },

    /// Show package information
    #[command(name = "info")]
    Info {
        /// Package name
        package: String,
    },

    /// List packages
    #[command(name = "list")]
    List {
        /// List only installed packages
        #[arg(long)]
        installed: bool,

        /// List only available packages
        #[arg(long)]
        available: bool,
    },

    /// Synchronize package databases
    #[command(name = "sync")]
    Sync,

    /// Run system diagnostics
    #[command(name = "doctor")]
    Doctor,

    /// Show transaction history
    #[command(name = "history")]
    History {
        /// Number of entries to show
        #[arg(short = 'n', long)]
        limit: Option<usize>,
    },
}

impl Cli {
    pub fn execute(&self, context: &mut Context) -> Result<()> {
        if self.assume_yes {
            context.set_assume_yes(true);
        }

        if self.no_color {
            context.set_color(false);
        }

        match &self.command {
            Commands::Install { packages } => {
                commands::install::execute(context, packages.clone())
            }
            Commands::Remove { packages } => {
                commands::remove::execute(context, packages.clone())
            }
            Commands::Update => {
                commands::update::execute(context)
            }
            Commands::Upgrade { packages } => {
                commands::upgrade::execute(context, packages.clone())
            }
            Commands::Search { query } => {
                commands::search::execute(context, query)
            }
            Commands::Info { package } => {
                commands::info::execute(context, package)
            }
            Commands::List { installed, available } => {
                commands::list::execute(context, *installed, *available)
            }
            Commands::Sync => {
                commands::sync::execute(context)
            }
            Commands::Doctor => {
                commands::doctor::execute(context)
            }
            Commands::History { limit } => {
                commands::history::execute(context, *limit)
            }
        }
    }
}
