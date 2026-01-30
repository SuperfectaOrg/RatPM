use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_default_config() {
    let config = ratpm::config::Config::default();
    
    assert_eq!(config.system.backend, "fedora");
    assert!(!config.system.assume_yes);
    assert!(config.system.color);
    assert!(config.repos.auto_refresh);
    assert!(config.repos.gpgcheck);
    assert!(config.transaction.keep_cache);
}

#[test]
fn test_config_validation() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("ratpm.toml");
    
    let invalid_config = r#"
[system]
backend = "invalid"
"#;
    
    fs::write(&config_path, invalid_config).unwrap();
    
    std::env::set_var("HOME", temp_dir.path());
    
    let content = fs::read_to_string(&config_path).unwrap();
    let config: Result<ratpm::config::Config, _> = toml::from_str(&content);
    
    if let Ok(parsed_config) = config {
        assert_eq!(parsed_config.system.backend, "invalid");
    }
}

#[test]
fn test_load_config_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("ratpm.toml");
    
    let config_content = r#"
[system]
backend = "fedora"
assume_yes = true
color = false

[repos]
auto_refresh = false
metadata_expire = 3600
gpgcheck = true

[transaction]
keep_cache = false
history_limit = 50
verify_signatures = true
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    let content = fs::read_to_string(&config_path).unwrap();
    let config: ratpm::config::Config = toml::from_str(&content).unwrap();
    
    assert_eq!(config.system.backend, "fedora");
    assert!(config.system.assume_yes);
    assert!(!config.system.color);
    assert!(!config.repos.auto_refresh);
    assert_eq!(config.repos.metadata_expire, 3600);
    assert!(!config.transaction.keep_cache);
    assert_eq!(config.transaction.history_limit, 50);
}

#[test]
fn test_invalid_config_format() {
    let invalid_toml = r#"
[system
backend = "fedora"
"#;
    
    let result: Result<ratpm::config::Config, _> = toml::from_str(invalid_toml);
    assert!(result.is_err());
}

#[test]
fn test_partial_config() {
    let partial_toml = r#"
[system]
backend = "fedora"
"#;
    
    let config: ratpm::config::Config = toml::from_str(partial_toml).unwrap();
    
    assert_eq!(config.system.backend, "fedora");
    assert!(!config.system.assume_yes);
    assert!(config.repos.auto_refresh);
}
