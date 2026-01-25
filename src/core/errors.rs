use thiserror::Error;

#[derive(Debug, Error)]
pub enum RatpmError {
    #[error("Permission denied: operation requires root privileges")]
    PermissionDenied,
    
    #[error("Package manager lock is held by another process (PID: {0})")]
    LockHeld(String),
    
    #[error("Lock acquisition timed out")]
    LockTimeout,
    
    #[error("Repository '{0}' is unavailable")]
    RepoUnavailable(String),
    
    #[error("Repository '{0}' failed GPG verification")]
    RepoGpgFailed(String),
    
    #[error("Dependency conflict: {0}")]
    DependencyConflict(String),
    
    #[error("Package '{0}' not found")]
    PackageNotFound(String),
    
    #[error("Package '{0}' is already installed")]
    PackageAlreadyInstalled(String),
    
    #[error("Package '{0}' is not installed")]
    PackageNotInstalled(String),
    
    #[error("Transaction check failed: {0}")]
    TransactionCheckFailed(String),
    
    #[error("Transaction execution failed: {0}")]
    TransactionFailed(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("RPM database error: {0}")]
    RpmDbError(String),
    
    #[error("Backend error: {0}")]
    BackendError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid package specification: {0}")]
    InvalidPackageSpec(String),
    
    #[error("Disk space insufficient: need {need} bytes, have {available} bytes")]
    InsufficientDiskSpace { need: u64, available: u64 },
    
    #[error("Scriptlet execution failed for package '{package}': {details}")]
    ScriptletFailed { package: String, details: String },
}

impl RatpmError {
    pub fn exit_code(&self) -> i32 {
        match self {
            RatpmError::PermissionDenied => 13,
            RatpmError::LockHeld(_) | RatpmError::LockTimeout => 14,
            RatpmError::PackageNotFound(_) => 1,
            RatpmError::DependencyConflict(_) => 2,
            RatpmError::TransactionCheckFailed(_) => 3,
            RatpmError::TransactionFailed(_) => 4,
            RatpmError::NetworkError(_) => 5,
            RatpmError::RepoUnavailable(_) | RatpmError::RepoGpgFailed(_) => 6,
            RatpmError::InsufficientDiskSpace { .. } => 7,
            RatpmError::ConfigError(_) => 8,
            RatpmError::RpmDbError(_) => 9,
            RatpmError::ScriptletFailed { .. } => 10,
            _ => 1,
        }
    }
}
