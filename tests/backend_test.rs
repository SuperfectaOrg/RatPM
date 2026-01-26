use ratpm::backend::fedora::FedoraBackend;
use ratpm::config::Config;

#[test]
fn test_backend_initialization() {
    let config = Config::default();
    let result = FedoraBackend::new(&config);
    assert!(result.is_ok());
}

#[test]
fn test_backend_search() {
    let config = Config::default();
    let backend = FedoraBackend::new(&config).unwrap();
    
    let results = backend.search("vim");
    assert!(results.is_ok());
}

#[test]
fn test_backend_list_installed() {
    let config = Config::default();
    let backend = FedoraBackend::new(&config).unwrap();
    
    let packages = backend.list_installed();
    assert!(packages.is_ok());
}

#[test]
fn test_backend_get_package_info() {
    let config = Config::default();
    let backend = FedoraBackend::new(&config).unwrap();
    
    let result = backend.get_package_info("bash");
    assert!(result.is_ok());
}

#[test]
fn test_backend_package_not_found() {
    let config = Config::default();
    let backend = FedoraBackend::new(&config).unwrap();
    
    let result = backend.get_package_info("nonexistent-package-xyz123");
    assert!(result.is_err());
}
