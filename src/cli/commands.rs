pub mod install {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::{Context as _, Result};

    pub fn execute(context: &mut Context, packages: Vec<String>) -> Result<()> {
        if packages.is_empty() {
            anyhow::bail!("No packages specified");
        }

        context
            .require_root()
            .context("Root privileges required for package installation")?;

        let lock = context
            .acquire_lock()
            .context("Failed to acquire package manager lock")?;

        let transaction = context
            .backend_mut()
            .resolve_install(&packages)
            .context("Failed to resolve dependencies")?;

        if transaction.is_empty() {
            output::info("Nothing to do");
            return Ok(());
        }

        output::print_transaction_summary(&transaction, context.color_enabled());

        if !context.confirm_transaction()? {
            output::info("Operation cancelled");
            return Ok(());
        }

        context
            .backend_mut()
            .execute(transaction)
            .context("Transaction failed")?;

        drop(lock);

        output::success("Transaction completed successfully");
        Ok(())
    }
}

pub mod remove {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::{Context as _, Result};

    pub fn execute(context: &mut Context, packages: Vec<String>) -> Result<()> {
        if packages.is_empty() {
            anyhow::bail!("No packages specified");
        }

        context
            .require_root()
            .context("Root privileges required for package removal")?;

        let lock = context
            .acquire_lock()
            .context("Failed to acquire package manager lock")?;

        let transaction = context
            .backend_mut()
            .resolve_remove(&packages)
            .context("Failed to resolve removal")?;

        if transaction.is_empty() {
            output::info("Nothing to do");
            return Ok(());
        }

        output::print_transaction_summary(&transaction, context.color_enabled());

        if !context.confirm_transaction()? {
            output::info("Operation cancelled");
            return Ok(());
        }

        context
            .backend_mut()
            .execute(transaction)
            .context("Transaction failed")?;

        drop(lock);

        output::success("Transaction completed successfully");
        Ok(())
    }
}

pub mod update {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::{Context as _, Result};

    pub fn execute(context: &mut Context) -> Result<()> {
        context
            .require_root()
            .context("Root privileges required for repository update")?;

        let lock = context
            .acquire_lock()
            .context("Failed to acquire package manager lock")?;

        output::info("Updating repository metadata...");

        context
            .backend_mut()
            .refresh_repositories()
            .context("Failed to refresh repositories")?;

        drop(lock);

        output::success("Repository metadata updated");
        Ok(())
    }
}

pub mod upgrade {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::{Context as _, Result};

    pub fn execute(context: &mut Context, packages: Option<Vec<String>>) -> Result<()> {
        context
            .require_root()
            .context("Root privileges required for system upgrade")?;

        let lock = context
            .acquire_lock()
            .context("Failed to acquire package manager lock")?;

        let transaction = match packages {
            Some(pkgs) => context.backend_mut().resolve_upgrade_packages(&pkgs)?,
            None => context.backend_mut().resolve_upgrade()?,
        };

        if transaction.is_empty() {
            output::info("Nothing to do");
            return Ok(());
        }

        output::print_transaction_summary(&transaction, context.color_enabled());

        if !context.confirm_transaction()? {
            output::info("Operation cancelled");
            return Ok(());
        }

        context
            .backend_mut()
            .execute(transaction)
            .context("Transaction failed")?;

        drop(lock);

        output::success("Upgrade completed successfully");
        Ok(())
    }
}

pub mod search {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::Result;

    pub fn execute(context: &mut Context, query: &str) -> Result<()> {
        let results = context.backend().search(query)?;

        if results.is_empty() {
            output::info(&format!("No packages found matching '{}'", query));
            return Ok(());
        }

        output::print_package_list(&results, context.color_enabled());
        Ok(())
    }
}

pub mod info {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::Result;

    pub fn execute(context: &mut Context, package: &str) -> Result<()> {
        let pkg = context.backend().get_package_info(package)?;
        output::print_package_info(&pkg, context.color_enabled());
        Ok(())
    }
}

pub mod list {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::Result;

    pub fn execute(context: &mut Context, installed: bool, available: bool) -> Result<()> {
        let packages = if installed {
            context.backend().list_installed()?
        } else if available {
            context.backend().list_available()?
        } else {
            context.backend().list_all()?
        };

        output::print_package_list(&packages, context.color_enabled());
        Ok(())
    }
}

pub mod sync {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::{Context as _, Result};

    pub fn execute(context: &mut Context) -> Result<()> {
        context
            .require_root()
            .context("Root privileges required for sync operation")?;

        let lock = context
            .acquire_lock()
            .context("Failed to acquire package manager lock")?;

        output::info("Synchronizing package databases...");

        context
            .backend_mut()
            .sync_databases()
            .context("Failed to synchronize databases")?;

        drop(lock);

        output::success("Databases synchronized");
        Ok(())
    }
}

pub mod doctor {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::Result;

    pub fn execute(context: &mut Context) -> Result<()> {
        output::info("Running system diagnostics...");

        let issues = context.backend().run_diagnostics()?;

        if issues.is_empty() {
            output::success("No issues found");
        } else {
            output::print_diagnostic_issues(&issues, context.color_enabled());
        }

        Ok(())
    }
}

pub mod history {
    use crate::cli::output;
    use crate::core::context::Context;
    use anyhow::Result;

    pub fn execute(context: &mut Context, limit: Option<usize>) -> Result<()> {
        let entries = context.backend().get_history(limit.unwrap_or(20))?;

        if entries.is_empty() {
            output::info("No transaction history");
            return Ok(());
        }

        output::print_history(&entries, context.color_enabled());
        Ok(())
    }
}
