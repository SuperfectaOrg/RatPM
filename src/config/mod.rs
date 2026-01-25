use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

mod schema;

const SYSTEM_CONFIG_PATH: &str = "/etc/ratpm/ratpm.toml";
const USER_CONFIG_PATH: &str = ".config/ratpm/ratpm.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub system: SystemConfig,
    
    #[serde(default)]
    pub repos: RepoConfig,
    
    #[serde(default)]
    pub transaction: TransactionConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemConfig {
    #[serde(default = "default_backend")]
    pub backend: String,
    
    #[serde(default)]
    pub assume_yes: bool,
    
    #[serde(default = "default_true")]
    pub color: bool,
    
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
    
    #[serde(default = "default_lock_file")]
    pub lock_file: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoConfig {
    #[serde(default = "default_true")]
    pub auto_refresh: bool,
    
    #[serde(default = "default_metadata_expire")]
    pub metadata_expire: u64,
    
    #[serde(default = "default_repo_dir")]
    pub repo_dir: PathBuf,
    
    #[serde(default = "default_true")]
    pub gpgcheck: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionConfig {
    #[serde(default = "default_true")]
    pub keep_cache: bool,
    
    #[serde(default = "default_history_limit")]
    pub history_limit: usize,
    
    #[serde(default = "default_true")]
    pub verify_signatures: bool,
}

fn default_backend() -> String {
    "fedora".to_string()
}

fn default_true() -> bool {
    true
}

fn default_cache_dir() -> PathBuf {
    PathBuf::from("/var/cache/ratpm")
}

fn default_lock_file() -> PathBuf {
    PathBuf::from("/var/lock/ratpm.lock")
}

fn default_metadata_expire() -> u64 {
    86400
}

fn default_repo_dir() -> PathBuf {
    PathBuf::from("/etc/yum.repos.d")
}

fn default_history_limit() -> usize {
    100
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            assume_yes: false,
            color: default_true(),
            cache_dir: default_cache_dir(),
            lock_file: default_lock_file(),
        }
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            auto_refresh: default_true(),
            metadata_expire: default_metadata_expire(),
            repo_dir: default_repo_dir(),
            gpgcheck: default_true(),
        }
    }
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            keep_cache: default_true(),
            history_limit: default_history_limit(),
            verify_signatures: default_true(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            repos: RepoConfig::default(),
            transaction: TransactionConfig::default(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let system_config = load_config_file(Path::new(SYSTEM_CONFIG_PATH));
    
    let user_config_path = dirs::home_dir()
        .map(|home| home.join(USER_CONFIG_PATH));
    
    let user_config = user_config_path
        .as_ref()
        .and_then(|path| load_config_file(path));
    
    match (system_config, user_config) {
        (Some(mut sys), Some(user)) => {
            merge_configs(&mut sys, user);
            validate_config(&sys)?;
            Ok(sys)
        }
        (Some(sys), None) => {
            validate_config(&sys)?;
            Ok(sys)
        }
        (None, Some(user)) => {
            validate_config(&user)?;
            Ok(user)
        }
        (None, None) => {
            let default = Config::default();
            validate_config(&default)?;
            Ok(default)
        }
    }
}

fn load_config_file(path: &Path) -> Option<Config> {
    fs::read_to_string(path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
}

fn merge_configs(base: &mut Config, overlay: Config) {
    if overlay.system.assume_yes {
        base.system.assume_yes = true;
    }
    
    if !overlay.system.color {
        base.system.color = false;
    }
}

fn validate_config(config: &Config) -> Result<()> {
    if config.system.backend != "fedora" {
        anyhow::bail!("Unsupported backend: {}", config.system.backend);
    }
    
    if config.repos.metadata_expire == 0 {
        anyhow::bail!("metadata_expire must be greater than 0");
    }
    
    if config.transaction.history_limit == 0 {
        anyhow::bail!("history_limit must be greater than 0");
    }
    
    Ok(())
}
