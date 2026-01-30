use crate::backend::fedora::types::PackageSpec;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub install: Vec<PackageSpec>,
    pub remove: Vec<PackageSpec>,
    pub upgrade: Vec<(PackageSpec, PackageSpec)>,
    pub download_size: u64,
    pub install_size: i64,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            install: Vec::new(),
            remove: Vec::new(),
            upgrade: Vec::new(),
            download_size: 0,
            install_size: 0,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.install.is_empty() && self.remove.is_empty() && self.upgrade.is_empty()
    }
    
    pub fn total_packages(&self) -> usize {
        self.install.len() + self.remove.len() + self.upgrade.len()
    }
    
    pub fn add_install(&mut self, package: PackageSpec, size: u64) {
        self.download_size += size;
        self.install_size = self.install_size.saturating_add(size.min(i64::MAX as u64) as i64);
        self.install.push(package);
    }
    
    pub fn add_remove(&mut self, package: PackageSpec, size: u64) {
        self.install_size = self.install_size.saturating_sub(size.min(i64::MAX as u64) as i64);
        self.remove.push(package);
    }
    
    pub fn add_upgrade(&mut self, old: PackageSpec, new: PackageSpec, old_size: u64, new_size: u64) {
        self.download_size += new_size;
        let new_i64 = new_size.min(i64::MAX as u64) as i64;
        let old_i64 = old_size.min(i64::MAX as u64) as i64;
        self.install_size = self.install_size.saturating_add(new_i64).saturating_sub(old_i64);
        self.upgrade.push((old, new));
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Pending,
    Resolving,
    Downloading,
    Verifying,
    Testing,
    Executing,
    Complete,
    Failed,
    Cancelled,
}

pub struct TransactionProgress {
    pub state: TransactionState,
    pub current_package: Option<String>,
    pub packages_processed: usize,
    pub total_packages: usize,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
}

impl TransactionProgress {
    pub fn new(total_packages: usize, total_bytes: u64) -> Self {
        Self {
            state: TransactionState::Pending,
            current_package: None,
            packages_processed: 0,
            total_packages,
            bytes_downloaded: 0,
            total_bytes,
        }
    }
    
    pub fn percentage(&self) -> f64 {
        if self.total_packages == 0 {
            return 0.0;
        }
        (self.packages_processed as f64 / self.total_packages as f64) * 100.0
    }
}
