use anyhow::Result;
use std::sync::Arc;
use crate::config::Config;
use crate::core::lock::{LockManager, LockGuard, FileLockManager};
use crate::core::errors::RatpmError;
use crate::backend::fedora::FedoraBackend;
use crate::cli::output;

pub struct Context {
    config: Config,
    backend: FedoraBackend,
    lock_manager: Arc<FileLockManager>,
    assume_yes: bool,
    color: bool,
    is_root: bool,
}

impl Context {
    pub fn new(config: Config) -> Result<Self> {
        let is_root = nix::unistd::geteuid().is_root();
        
        let lock_manager = Arc::new(FileLockManager::new(
            config.system.lock_file.clone()
        ));
        
        let backend = FedoraBackend::new(&config)?;
        
        Ok(Self {
            assume_yes: config.system.assume_yes,
            color: config.system.color,
            backend,
            lock_manager,
            config,
            is_root,
        })
    }
    
    pub fn require_root(&self) -> Result<(), RatpmError> {
        if !self.is_root {
            return Err(RatpmError::PermissionDenied);
        }
        Ok(())
    }
    
    pub fn acquire_lock(&self) -> Result<LockGuard> {
        self.lock_manager.acquire()
    }
    
    pub fn backend(&self) -> &FedoraBackend {
        &self.backend
    }
    
    pub fn backend_mut(&mut self) -> &mut FedoraBackend {
        &mut self.backend
    }
    
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    pub fn set_assume_yes(&mut self, value: bool) {
        self.assume_yes = value;
    }
    
    pub fn set_color(&mut self, value: bool) {
        self.color = value;
    }
    
    pub fn color_enabled(&self) -> bool {
        self.color
    }
    
    pub fn confirm_transaction(&self) -> Result<bool> {
        if self.assume_yes {
            return Ok(true);
        }
        
        output::prompt_confirmation("Proceed with transaction?")
            .map_err(|e| anyhow::anyhow!("Failed to read user input: {}", e))
    }
}
