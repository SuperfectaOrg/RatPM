use std::process::Command;

#[test]
fn test_ratpm_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ratpm"));
}

#[test]
fn test_ratpm_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RatOS Package Manager"));
    assert!(stdout.contains("install"));
    assert!(stdout.contains("remove"));
    assert!(stdout.contains("update"));
}

#[test]
fn test_ratpm_search_no_root() {
    let output = Command::new("cargo")
        .args(&["run", "--", "search", "vim"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(output.status.success());
}

#[test]
fn test_ratpm_info_no_root() {
    let output = Command::new("cargo")
        .args(&["run", "--", "info", "bash"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(output.status.success());
}

#[test]
fn test_ratpm_list_no_root() {
    let output = Command::new("cargo")
        .args(&["run", "--", "list", "--installed"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(output.status.success());
}

#[cfg(unix)]
#[test]
fn test_ratpm_install_requires_root() {
    use std::os::unix::process::ExitStatusExt;
    
    let output = Command::new("cargo")
        .args(&["run", "--", "install", "vim"])
        .output()
        .expect("Failed to execute ratpm");
    
    if !nix::unistd::geteuid().is_root() {
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Permission denied") || stderr.contains("Root privileges"));
        
        if let Some(code) = output.status.code() {
            assert_eq!(code, 13);
        }
    }
}

#[test]
fn test_invalid_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "nonexistent"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(!output.status.success());
}

#[test]
fn test_install_no_packages() {
    let output = Command::new("cargo")
        .args(&["run", "--", "install"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(!output.status.success());
}

#[test]
fn test_remove_no_packages() {
    let output = Command::new("cargo")
        .args(&["run", "--", "remove"])
        .output()
        .expect("Failed to execute ratpm");
    
    assert!(!output.status.success());
}
