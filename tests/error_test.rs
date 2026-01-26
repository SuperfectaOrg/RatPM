use ratpm::core::errors::RatpmError;

#[test]
fn test_error_exit_codes() {
    assert_eq!(RatpmError::PermissionDenied.exit_code(), 13);
    assert_eq!(RatpmError::LockHeld("123".to_string()).exit_code(), 14);
    assert_eq!(RatpmError::PackageNotFound("vim".to_string()).exit_code(), 1);
    assert_eq!(RatpmError::DependencyConflict("conflict".to_string()).exit_code(), 2);
    assert_eq!(RatpmError::TransactionCheckFailed("check".to_string()).exit_code(), 3);
    assert_eq!(RatpmError::TransactionFailed("failed".to_string()).exit_code(), 4);
    assert_eq!(RatpmError::NetworkError("network".to_string()).exit_code(), 5);
    assert_eq!(RatpmError::RepoUnavailable("repo".to_string()).exit_code(), 6);
    assert_eq!(RatpmError::ConfigError("config".to_string()).exit_code(), 8);
    assert_eq!(RatpmError::RpmDbError("db".to_string()).exit_code(), 9);
}

#[test]
fn test_error_messages() {
    let err = RatpmError::PermissionDenied;
    assert_eq!(err.to_string(), "Permission denied: operation requires root privileges");
    
    let err = RatpmError::PackageNotFound("neovim".to_string());
    assert_eq!(err.to_string(), "Package 'neovim' not found");
    
    let err = RatpmError::LockHeld("1234".to_string());
    assert_eq!(err.to_string(), "Package manager lock is held by another process (PID: 1234)");
}

#[test]
fn test_error_types() {
    let err = RatpmError::PackageAlreadyInstalled("vim".to_string());
    assert!(matches!(err, RatpmError::PackageAlreadyInstalled(_)));
    
    let err = RatpmError::PackageNotInstalled("vim".to_string());
    assert!(matches!(err, RatpmError::PackageNotInstalled(_)));
}

#[test]
fn test_insufficient_disk_space_error() {
    let err = RatpmError::InsufficientDiskSpace { 
        need: 1_000_000_000, 
        available: 500_000_000 
    };
    assert_eq!(err.exit_code(), 7);
    assert!(err.to_string().contains("Disk space insufficient"));
}

#[test]
fn test_scriptlet_failed_error() {
    let err = RatpmError::ScriptletFailed {
        package: "vim".to_string(),
        details: "post install failed".to_string(),
    };
    assert_eq!(err.exit_code(), 10);
    assert!(err.to_string().contains("Scriptlet execution failed"));
    assert!(err.to_string().contains("vim"));
}
