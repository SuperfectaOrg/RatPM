use crate::config::Config;
use crate::core::errors::RatpmError;
use crate::core::transaction::Transaction;
use anyhow::{Context as _, Result};

mod libdnf;
mod repos;
mod rpm;
mod transaction;
pub mod types;

pub use types::{DiagnosticIssue, HistoryEntry, Package, PackageInfo};

pub struct FedoraBackend {
    config: Config,
    repos: repos::RepositoryManager,
    rpm_db: rpm::RpmDatabase,
    cache_dir: std::path::PathBuf,
}

impl FedoraBackend {
    pub fn new(config: &Config) -> Result<Self> {
        let cache_dir = config.system.cache_dir.clone();

        std::fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;

        let repos = repos::RepositoryManager::new(
            config.repos.repo_dir.clone(),
            cache_dir.clone(),
            config.repos.gpgcheck,
        )?;

        let rpm_db = rpm::RpmDatabase::new()?;

        Ok(Self {
            config: config.clone(),
            repos,
            rpm_db,
            cache_dir,
        })
    }

    pub fn search(&self, query: &str) -> Result<Vec<Package>> {
        self.repos.search(query)
    }

    pub fn resolve_install(&self, packages: &[String]) -> Result<Transaction> {
        libdnf::resolve_install(&self.repos, &self.rpm_db, packages)
    }

    pub fn resolve_remove(&self, packages: &[String]) -> Result<Transaction> {
        for pkg_name in packages {
            if !self.rpm_db.is_installed(pkg_name)? {
                return Err(RatpmError::PackageNotInstalled(pkg_name.clone()).into());
            }
        }

        libdnf::resolve_remove(&self.rpm_db, packages)
    }

    pub fn resolve_upgrade(&self) -> Result<Transaction> {
        libdnf::resolve_upgrade(&self.repos, &self.rpm_db)
    }

    pub fn resolve_upgrade_packages(&self, packages: &[String]) -> Result<Transaction> {
        libdnf::resolve_upgrade_packages(&self.repos, &self.rpm_db, packages)
    }

    pub fn execute(&mut self, transaction: Transaction) -> Result<()> {
        transaction::execute_transaction(
            &self.repos,
            &mut self.rpm_db,
            &self.cache_dir,
            transaction,
            self.config.transaction.verify_signatures,
        )
    }

    pub fn get_package_info(&self, package: &str) -> Result<PackageInfo> {
        if let Ok(info) = self.rpm_db.get_package_info(package) {
            return Ok(info);
        }

        self.repos.get_package_info(package)
    }

    pub fn list_installed(&self) -> Result<Vec<Package>> {
        self.rpm_db.list_all()
    }

    pub fn list_available(&self) -> Result<Vec<Package>> {
        self.repos.list_available()
    }

    pub fn list_all(&self) -> Result<Vec<Package>> {
        let mut packages = self.list_installed()?;
        packages.extend(self.list_available()?);
        packages.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.version.cmp(&b.version))
                .then_with(|| a.arch.cmp(&b.arch))
        });
        packages.dedup_by(|a, b| a.name == b.name && a.version == b.version && a.arch == b.arch);
        Ok(packages)
    }

    pub fn refresh_repositories(&mut self) -> Result<()> {
        self.repos.refresh_metadata()
    }

    pub fn sync_databases(&mut self) -> Result<()> {
        self.repos.sync_all()
    }

    pub fn run_diagnostics(&self) -> Result<Vec<DiagnosticIssue>> {
        let mut issues = Vec::new();

        if let Err(e) = self.rpm_db.verify_integrity() {
            issues.push(DiagnosticIssue {
                severity: "error".to_string(),
                message: format!("RPM database integrity check failed: {}", e),
                suggestion: "Run 'rpm --rebuilddb' to rebuild the database".to_string(),
            });
        }

        let repo_issues = self.repos.check_health()?;
        issues.extend(repo_issues);

        Ok(issues)
    }

    pub fn get_history(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        self.rpm_db.get_transaction_history(limit)
    }
}
